use askama::Template;
#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactView {
    pub name: String,
    pub phone_number: String,
    pub email: String,
}
