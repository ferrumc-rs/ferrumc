use crate::{
    PersistentDataHolder, PersistentKey, container::PersistentDataContainer,
    errors::PersistentDataError,
};

#[derive(Default)]
struct PlayerTesting {
    data: PersistentDataContainer,
}

impl PersistentDataHolder for PlayerTesting {
    fn get_persistent_data(&self) -> &PersistentDataContainer {
        &self.data
    }

    fn edit_persistent_data<
        F: FnOnce(&mut PersistentDataContainer) -> Result<(), PersistentDataError>,
    >(
        &mut self,
        func: F,
    ) {
        func(&mut self.data).expect("Editing Persistent Data failed for PlayerTesting");
    }
}

struct TestingKey;

impl PersistentKey for TestingKey {
    type Value = i32;

    fn key() -> &'static str {
        "health"
    }
}

#[test]
fn something() -> Result<(), PersistentDataError> {
    let mut testing = PlayerTesting::default();
    testing.edit_persistent_data(|data| {
        data.set::<TestingKey>(100)?;

        Ok(())
    });

    let persistent_data = testing.get_persistent_data();
    if let Ok(grabbed_value) = persistent_data.get::<TestingKey>() {
        println!("Testing: {}", grabbed_value);
    }

    Ok(())
}
