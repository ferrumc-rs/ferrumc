use indexmap::IndexMap;
use quote::quote;
use serde_json::Value;

pub(crate) fn build_mapping(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let json_file = include_bytes!("../../../../../assets/data/registry_packets.json");
    let val: IndexMap<String, IndexMap<String, Value>> = serde_json::from_slice(json_file).unwrap();

    let mut registry_entries = vec![];

    for (reg_entry, value_set) in val {
        let mut packets = vec![];
        for (value_name, value) in &value_set {
            let mut nbt_data_buf = Vec::new();
            craftflow_nbt::to_writer(&mut nbt_data_buf, &value).unwrap();
            let kv = (value_name.clone(), nbt_data_buf);
            packets.push(kv);
        }
        registry_entries.push((reg_entry, packets));
    }
    let pairs = registry_entries
        .iter()
        .map(|(key, packets)| {
            let raw_packets_data = bitcode::encode(packets);
            quote! {
                (#key.to_string(), vec![#(#raw_packets_data),*])
            }
        })
        .collect::<Vec<_>>();

    quote! {
        indexmap::IndexMap::from([
            #(#pairs),*
        ])
    }
        .into()
}
