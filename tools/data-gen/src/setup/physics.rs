use super::MC_VERSION;
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const DECOMPILER_URL: &str = "https://github.com/MaxPixelStudios/MinecraftDecompiler/releases/download/v3.3.2/MinecraftDecompiler.jar";

// Embed the Java source code so it's compiled into the Rust binary
const BLOCK_DUMPER_SRC: &str = include_str!("../../java_src/BlockExtractor.java");
const MAPPING_EXTRACTOR_SRC: &str = include_str!("../../java_src/MappingExtractor.java");

pub fn prepare_remapped_jar(temp_dir: &Path) -> PathBuf {
    let remapped_jar = temp_dir.join(format!("server-{}-remapped.jar", MC_VERSION));

    if remapped_jar.exists() {
        println!("Found cached remapped JAR.");
        return remapped_jar;
    }

    println!("Remapped JAR not found. Starting Decompilation process...");

    let decompiler_path = temp_dir.join("MinecraftDecompiler.jar");
    if !decompiler_path.exists() {
        println!("Downloading Decompiler...");
        let mut resp = Client::new()
            .get(DECOMPILER_URL)
            .send()
            .expect("Failed to download decompiler");
        let mut file = fs::File::create(&decompiler_path).unwrap();
        resp.copy_to(&mut file).unwrap();
    }

    println!("Running MinecraftDecompiler (This will take a while)...");
    let status = Command::new("java")
        .current_dir(temp_dir)
        .arg("-jar")
        .arg(&decompiler_path)
        .arg("--version")
        .arg(MC_VERSION)
        .arg("--side")
        .arg("SERVER")
        .arg("--decompile")
        .arg("--output")
        .arg(&remapped_jar)
        .arg("--decompiled-output")
        .arg(temp_dir.join("src_out"))
        .status()
        .expect("Failed to run Decompiler");

    if !status.success() {
        panic!("Decompilation failed!");
    }

    if !remapped_jar.exists() {
        panic!("Decompiler finished but remapped JAR is missing!");
    }

    remapped_jar
}

pub fn extract_physics_data(remapped_jar: &Path, temp_dir: &Path) -> PathBuf {
    let json_path = temp_dir.join("physics.json");

    if json_path.exists() {
        println!("Found cached Physics Data. Skipping extraction.");
        return json_path;
    }

    run_java_tool(
        "BlockInfoDumper",
        BLOCK_DUMPER_SRC,
        remapped_jar,
        temp_dir,
        &json_path,
    );

    json_path
}

pub fn extract_mappings(remapped_jar: &Path, temp_dir: &Path) -> PathBuf {
    let json_path = temp_dir.join("mappings.json");

    if json_path.exists() {
        println!("Found cached Mappings. Skipping extraction.");
        return json_path;
    }

    // Reuse the helper function we wrote earlier
    run_java_tool(
        "MappingExtractor",
        MAPPING_EXTRACTOR_SRC,
        remapped_jar,
        temp_dir,
        &json_path,
    );

    json_path
}

// Helper function for compiling and running the java extractors
fn run_java_tool(
    class_name: &str,
    source_code: &str,
    remapped_jar: &Path,
    temp_dir: &Path,
    output_file: &Path,
) {
    println!("Compiling {}...", class_name);

    // 1. Write Java Source
    let java_file = temp_dir.join(format!("{}.java", class_name));
    fs::write(&java_file, source_code).unwrap();

    // 2. Build Classpath
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

    println!("Running {}...", class_name);

    // 4. Run
    // We pass the output file path as the first argument to the Java program
    let output_abs_path = output_file
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap()
        .join(output_file.file_name().unwrap());

    let output = Command::new("java")
        .current_dir(temp_dir)
        .arg("-cp")
        .arg(&classpath)
        .arg(class_name)
        .arg(&output_abs_path) // Arg[0] for the Java program
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

    println!("{} finished successfully.", class_name);
}
