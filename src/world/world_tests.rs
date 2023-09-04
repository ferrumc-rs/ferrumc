use std::collections::HashMap;
use std::io::Read;
use flate2;
use serde_json::Value;
use crate::world::nbtstructs::WorldData;

#[derive(Debug, serde::Deserialize)]
struct GenericData {

    #[serde(flatten, rename = "Data")]
    Data: HashMap<String, Value>
}
#[test]
fn test_world() {
    let mut file = std::fs::File::open("C:\\Users\\ReCor\\Documents\\Paper\\world\\level.dat").unwrap();
    //println!("{:#?}", file.read(&mut [0; 100]).is_ok());
    let bufreader = std::io::BufReader::new(file);
    let reader = flate2::read::MultiGzDecoder::new(bufreader);
    let world: WorldData = fastnbt::from_reader(reader).unwrap();
    // if let Ok(world) = world {
    //     println!("{:#?}", world);
    // } else {
    //     panic!("World data could not be read!")
    // }
}