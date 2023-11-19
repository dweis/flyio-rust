use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    password: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as!(
            User,
            r#"
                    select * from users where email = $1
                "#,
            creds.email
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user.filter(|user| {
            verify_password(creds.password, &user.password)
                .ok()
                .is_some() // We're using password-based authentication--this
                           // works by comparing our form input with an argon2
                           // password hash.
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                    select user_id, email, password, created_at
                    from users
                    where user_id = $1
                "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;

pub async fn create_user(db: &PgPool, email: &str, password: &str) -> Result<User, sqlx::Error> {
    let password = password_auth::generate_hash(password);

    let user = sqlx::query_as!(
        User,
        r#"
                with inserted_user as (
                    insert into users (email, password)
                    values ($1, $2)
                    returning user_id, email, password, created_at
                )
                select user_id, email, password, created_at from inserted_user
            "#,
        email,
        password
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}
