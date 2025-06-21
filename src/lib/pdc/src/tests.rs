use crate::{PersistentKey, container::PersistentDataContainer};

#[test]
fn something() {
    let testing_key = PersistentKey::<i32>::new("minecraft", "testing");
    let mut persistent_data = PersistentDataContainer::default();

    persistent_data.set(&testing_key, 10);

    match persistent_data.get(&testing_key) {
        Ok(testing_value) => {
            println!("Testing: {}", testing_value);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
