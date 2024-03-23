use crate::types::contact::Contact;
use crate::types::contacts_view::ContactsView;
use crate::types::index_view::IndexView;
pub async fn index<'a>() -> IndexView<'a> {
    IndexView {
        contacts_view: ContactsView {
            contacts: vec![Contact {
                name: "<script>console.log('foo')</script>".into(),
                phone_number: "bar".into(),
                email: "baz".into(),
            }],
        },
    }
}
