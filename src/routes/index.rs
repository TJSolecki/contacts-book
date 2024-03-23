use crate::types::contact::Contact;
use crate::types::contacts_view::ContactsView;
use crate::types::index_view::IndexView;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::SqlitePool;

#[axum_macros::debug_handler]
pub async fn index(State(pool): State<SqlitePool>) -> Result<IndexView, StatusCode> {
    let contacts: Vec<Contact> = sqlx::query_as!(
        Contact,
        r#"
SELECT name, phone_number, email
FROM contacts
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(IndexView {
        contacts_view: ContactsView { contacts },
    })
}
