use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

// Embed the Java sources here
const BLOCK_EXTRACTOR_SRC: &str = include_str!("../../java_src/BlockExtractor.java");
const MAPPING_EXTRACTOR_SRC: &str = include_str!("../../java_src/MappingExtractor.java");
const ENTITY_EXTRACTOR_SRC: &str = include_str!("../../java_src/EntityExtractor.java");

pub fn extract_blocks(remapped_jar: &Path, temp_dir: &Path) -> PathBuf {
    let json_path = temp_dir.join("extracted_blocks.json");
    if json_path.exists() {
        println!("Found cached Data. Skipping extraction.");
        return json_path;
    }

    run_java_tool(
        "BlockExtractor",
        BLOCK_EXTRACTOR_SRC,
        remapped_jar,
        temp_dir,
        &json_path,
    );
    json_path
}

pub fn extract_mappings(remapped_jar: &Path, temp_dir: &Path) -> PathBuf {
    let json_path = temp_dir.join("mappings.json");
    if json_path.exists() {
        println!("Found cached Data. Skipping extraction.");
        return json_path;
    }

    run_java_tool(
        "MappingExtractor",
        MAPPING_EXTRACTOR_SRC,
        remapped_jar,
        temp_dir,
        &json_path,
    );
    json_path
}

pub fn extract_entities(remapped_jar: &Path, temp_dir: &Path) -> PathBuf {
    let json_path = temp_dir.join("extracted_entities.json");

    // Cache Check
    if json_path.exists() {
        println!("Found cached Data. Skipping extraction.");
        return json_path;
    }

    run_java_tool(
        "EntityExtractor",
        ENTITY_EXTRACTOR_SRC,
        remapped_jar,
        temp_dir,
        &json_path,
    );

    json_path
}

// --- Internal Helper ---

fn run_java_tool(
    class_name: &str,
    source_code: &str,
    remapped_jar: &Path,
    temp_dir: &Path,
    output_file: &Path,
) {
    println!("Compiling & Running {}...", class_name);

    // 1. Write Java Source
    let java_file = temp_dir.join(format!("{}.java", class_name));
    fs::write(&java_file, source_code).unwrap();

    // 2. Build Classpath (Jar + Libraries)
    let remapped_abs = remapped_jar.canonicalize().unwrap();
    let cp_sep = if cfg!(windows) { ";" } else { ":" };
    let mut classpath = format!(".{}{}", cp_sep, remapped_abs.display());

    let lib_dir = temp_dir.join("libraries");
    if lib_dir.exists() {
        for entry in WalkDir::new(&lib_dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "jar") {
                if let Ok(abs_path) = path.canonicalize() {
                    classpath.push_str(cp_sep);
                    classpath.push_str(&abs_path.to_string_lossy());
                }
            }
        }
    }

    // 3. Compile
    let compile_status = Command::new("javac")
        .current_dir(temp_dir)
        .arg("-cp")
        .arg(&classpath)
        .arg(format!("{}.java", class_name))
        .output()
        .expect("Failed to run javac");

    if !compile_status.status.success() {
        panic!(
            "Java compilation failed:\n{}",
            String::from_utf8_lossy(&compile_status.stderr)
        );
    }

    // 4. Run
    let output_abs_path = if output_file.is_absolute() {
        output_file.to_path_buf()
    } else {
        env::current_dir().unwrap().join(output_file)
    };

    let output = Command::new("java")
        .current_dir(temp_dir)
        .arg("-cp")
        .arg(&classpath)
        .arg(class_name)
        .arg(&output_abs_path)
        .output()
        .expect("Failed to run Java tool");

    if !output.status.success() {
        println!("Java execution failed!");
        println!(
            "--- STDOUT ---\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        println!(
            "--- STDERR ---\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Tool execution failed");
    }

    if !output_file.exists() {
        panic!(
            "Java tool finished but {} was not created!",
            output_file.display()
        );
    }
}
