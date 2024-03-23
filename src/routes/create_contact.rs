use crate::types::contact::Contact;
use crate::types::contact_view::ContactView;
use axum::extract::Form;
use axum::extract::State;
use axum::http::StatusCode;
use sqlx::Pool;
use sqlx::Sqlite;

#[axum_macros::debug_handler]
pub async fn create_contact(
    State(pool): State<Pool<Sqlite>>,
    Form(form_data): Form<Contact>,
) -> Result<ContactView, StatusCode> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert the task, then obtain the ID of this row
    sqlx::query!(
        r#"
            INSERT INTO contacts (name, phone_number, email)
            VALUES ( ?1, ?2, ?3 )
        "#,
        form_data.name,
        form_data.phone_number,
        form_data.email,
    )
    .execute(&mut *conn)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .last_insert_rowid();

    Ok(ContactView {
        name: form_data.name,
        phone_number: form_data.phone_number,
        email: form_data.email,
    })
}
