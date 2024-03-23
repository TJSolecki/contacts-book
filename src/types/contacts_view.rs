use crate::types::contact::Contact;
use askama_axum::Template;
#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactsView<'a> {
    pub contacts: Vec<Contact<'a>>,
}
