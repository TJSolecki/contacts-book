use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone_number: String,
    pub email: String,
}
