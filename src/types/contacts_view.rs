use crate::types::contact::Contact;
use askama_axum::Template;
#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactsView {
    pub contacts: Vec<Contact>,
}
