use crate::types::contacts_view::ContactsView;
use askama_axum::Template;
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexView<'a> {
    pub contacts_view: ContactsView<'a>,
}
