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
    // `OffsetDateTime`'s default serialization format is not standard.
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

pub async fn create_todo(db: &PgPool, content: String) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        r#"
            with inserted_todo as (
                insert into todo(content)
                values($1)
                returning todo_id, content, done, created_at
            ) 
            select todo_id, content, done, created_at from inserted_todo
        "#,
        content
    )
    .fetch_one(db)
    .await
}

pub async fn get_todos(db: &PgPool) -> Result<Vec<Todo>, Error> {
    sqlx::query_as!(
        Todo,
        "
            select todo_id, content, done, created_at
            from todo
            order by created_at
        ",
    )
    .fetch_all(db)
    .await
}

pub async fn get_todo_by_id(db: &PgPool, todo_id: Uuid) -> Result<Todo, Error> {
    sqlx::query_as!(
        Todo,
        "
            select todo_id, content, done, created_at
            from todo
            where todo_id = $1
        ",
        todo_id
    )
    .fetch_one(db)
    .await
}

pub async fn delete_todo_by_id(db: &PgPool, todo_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            delete from todo
            where todo_id = $1
        ",
        todo_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn toggle_todo_by_id(db: &PgPool, todo_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            update todo
            set done = not done
            where todo_id = $1
        ",
        todo_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_todo_by_id(db: &PgPool, todo_id: Uuid, content: String) -> Result<(), Error> {
    sqlx::query!(
        "
            update todo
            set content = $1
            where todo_id = $2
        ",
        content,
        todo_id
    )
    .execute(db)
    .await?;

    Ok(())
}
