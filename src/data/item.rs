use serde::Serialize;
use sqlx::{Error, PgPool};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[serde_with::serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_id: Uuid,
    pub content: String,
    // `OffsetDateTime`'s default serialization format is not standard.
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

pub async fn create_item(db: &PgPool, content: String) -> Result<Item, Error> {
    sqlx::query_as!(
        Item,
        r#"
            with inserted_item as (
                insert into item(content)
                values($1)
                returning item_id, content, created_at
            ) 
            select item_id, content, created_at from inserted_item
        "#,
        content
    )
    .fetch_one(&*db)
    .await
}

pub async fn get_items(db: &PgPool) -> Result<Vec<Item>, Error> {
    sqlx::query_as!(
        Item,
        // language=PostgreSQL
        "
            select item_id, content, created_at
            from item
            order by created_at
        ",
    )
    .fetch_all(&*db)
    .await
}
