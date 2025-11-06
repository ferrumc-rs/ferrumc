use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use simd_json::{
    self,
    base::{ValueAsObject, ValueAsScalar},
    derived::ValueTryAsObject,
};

const JSON_FILE: &[u8] = include_bytes!("../../assets/data/blockstates.json");

fn main() {
    let mut buf = JSON_FILE.to_owned();
    let v = simd_json::to_borrowed_value(&mut buf).unwrap();

    let mut out = vec![];
    for (k, v) in v.try_as_object().expect("object value") {
        let id = k;
        let block = v.as_object().unwrap();
        let name = block.get("name").unwrap().as_str().unwrap();
        let props = block.get("properties");
        if let Some(props) = props {
            out.push((
                id.parse::<u16>().unwrap(),
                format!(
                    "    assert_eq!(block!(\"{}\", {}), BlockId({}));",
                    name,
                    format_props(props),
                    id
                ),
            ));
        } else {
            out.push((
                id.parse::<u16>().unwrap(),
                format!("    assert_eq!(block!(\"{}\"), BlockId({}));", name, id),
            ));
        }
    }

    out.sort_by_key(|(k, _)| *k);
    let out = out.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    let path =
        Path::new(&env::var("OUT_DIR").expect("OUT_DIR env varible set")).join("block_test.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    for (i, chunk) in out.chunks(40).enumerate() {
        write!(
            &mut file,
            "#[ignore]\n#[test]\nfn all_the_blocks_{i}() {{\n{}\n}}\n\n",
            chunk.join("\n")
        )
        .expect("able to write to file");
    }
    println!("created {}", &path.to_string_lossy());
}

fn format_props(props: &simd_json::BorrowedValue) -> String {
    let props_str = props
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            let k_str = match k.as_ref() {
                "type" => "r#type".to_string(),
                _ => k.to_string(),
            };
            let v_str = match v {
                simd_json::BorrowedValue::Static(static_node) => match static_node {
                    simd_json::StaticNode::I64(v) => v.to_string(),
                    simd_json::StaticNode::U64(v) => v.to_string(),
                    simd_json::StaticNode::F64(v) => v.to_string(),
                    simd_json::StaticNode::Bool(v) => (match v {
                        true => "true",
                        false => "false",
                    })
                    .to_string(),
                    simd_json::StaticNode::Null => "\"null\"".to_string(),
                },
                simd_json::BorrowedValue::String(cow) => format!("\"{}\"", cow),
                _ => unreachable!(),
            };
            format!(" {}: {}", k_str, v_str)
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{}}}", props_str)
}
