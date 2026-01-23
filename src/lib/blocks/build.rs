fn main() {
    println!("cargo:rerun-if-changed=../../../assets/data/blockstates.json");
    // TODO: this should generate all block structs with the decoded ids as well as the mapping of block state ids to struct
}