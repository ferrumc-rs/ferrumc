use quote::quote;
use std::sync::LazyLock;
use syn::{parse_macro_input, LitStr};

static JSON_CONTENT: LazyLock<serde_json::Value> = LazyLock::new(|| {
    let json_str = include_str!("../../../../../assets/data/registries.json");
    serde_json::from_str(json_str).unwrap()
});

pub(crate) fn get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = parse_macro_input!(input as LitStr).value();
    let parts: Vec<&str> = path.split('.').collect();

    let mut current_value = &*JSON_CONTENT;

    for part in parts {
        // current_value = &current_value[part];
        current_value = current_value
            .get(part)
            .unwrap_or_else(|| panic!("Could not find key: {}", part));
    }

    let protocol_id = current_value
        .get("protocol_id")
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| panic!("Could not find key: {}", "protocol_id"));

    let expanded = quote! {
        #protocol_id
    };

    proc_macro::TokenStream::from(expanded)
}
