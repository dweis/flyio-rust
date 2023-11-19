use serde::Serialize;
use sqlx::{Error, PgPool};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub todo_id: Uuid,
    pub content: String,
    pub done: bool,
    pub user_id: Uuid,
    // `OffsetDateTime`'s default serialization format is not standard.
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

pub async fn create_todo(db: &PgPool, user_id: Uuid, content: String) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        r#"
            with inserted_todo as (
                insert into todos(user_id, content)
                values($1, $2)
                returning todo_id, content, done, user_id, created_at
            ) 
            select todo_id, content, done, user_id, created_at from inserted_todo
        "#,
        user_id,
        content,
    )
    .fetch_one(db)
    .await
}

pub async fn get_todos(db: &PgPool, user_id: Uuid) -> Result<Vec<Todo>, Error> {
    sqlx::query_as!(
        Todo,
        "
            select todo_id, content, done, user_id, created_at
            from todos
            where user_id = $1
            order by created_at
        ",
        user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn get_todo_by_id(db: &PgPool, user_id: Uuid, todo_id: Uuid) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        "
            select todo_id, content, done, user_id, created_at
            from todos
            where user_id = $1 and todo_id = $2
        ",
        user_id,
        todo_id,
    )
    .fetch_one(db)
    .await
}

pub async fn delete_todo_by_id(db: &PgPool, user_id: Uuid, todo_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            delete from todos
            where user_id = $1 and todo_id = $2
        ",
        user_id,
        todo_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn toggle_todo_by_id(db: &PgPool, user_id: Uuid, todo_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            update todos
            set done = not done
            where user_id = $1 and todo_id = $2
        ",
        user_id,
        todo_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_todo_by_id(
    db: &PgPool,
    user_id: Uuid,
    todo_id: Uuid,
    content: String,
) -> Result<(), Error> {
    sqlx::query!(
        "
            update todos
            set content = $3
            where user_id = $1 and todo_id = $2
        ",
        user_id,
        todo_id,
        content,
    )
    .execute(db)
    .await?;

    Ok(())
}
