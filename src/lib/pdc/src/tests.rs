use crate::{PersistentDataHolder, PersistentKey, container::PersistentDataContainer};

#[derive(Default)]
struct PlayerTesting {
    data: PersistentDataContainer,
}

impl PersistentDataHolder for PlayerTesting {
    fn get_persistent_data(&self) -> &PersistentDataContainer {
        &self.data
    }

    fn edit_persistent_data<F: FnOnce(&mut PersistentDataContainer)>(&mut self, func: F) {
        func(&mut self.data);
    }
}

#[test]
fn something() {
    let mut testing = PlayerTesting::default();
    let testing_key = PersistentKey::<i32>::new("minecraft", "testing");

    testing.edit_persistent_data(|data| {
        data.set(&testing_key, 100);
    });

    let persistent_data = testing.get_persistent_data();
    if let Ok(grabbed_value) = persistent_data.get(&testing_key) {
        println!("Testing: {}", grabbed_value);
    }
}
