use std::time::Instant;

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

#[test]
fn something() -> Result<(), PersistentDataError> {
    let instant = Instant::now();
    let key = PersistentKey::<i32>::new("testing");
    let mut testing = PlayerTesting::default();
    testing.edit_persistent_data(|data| {
        data.set(&key, 100)?;

        Ok(())
    });

    let persistent_data = testing.get_persistent_data();
    if let Some(grabbed_value) = persistent_data.get(&key) {
        println!("Testing 1: {}", grabbed_value);
    }

    // Or
    println!("Testing 2: {}", persistent_data.get_unchecked(&key));
    println!("Testing 3: {}", persistent_data.get_or(&key, 150));

    println!("Finished in: {:?}", instant.elapsed());

    Ok(())
}
