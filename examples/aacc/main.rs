use customer::{mod_wsd::ContactType, multi_ref};

mod customer;

#[tokio::main]
async fn main() {
    env_logger::init();
    // self-referencing struct
    let contact_type = ContactType {
        parent: Some(multi_ref::MultiRef::new(ContactType::default())),
        ..Default::default()
    };
    let xml = yaserde::ser::to_string(&contact_type).unwrap();
    println!("{xml}");
}
