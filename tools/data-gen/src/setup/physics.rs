use super::MC_VERSION;
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const DECOMPILER_URL: &str = "https://github.com/MaxPixelStudios/MinecraftDecompiler/releases/download/v3.3.2/MinecraftDecompiler.jar";

// Embed the Java source code so it's compiled into the Rust binary
const JAVA_DUMPER_SOURCE: &str = include_str!("../../java_src/BlockInfoDumper.java");

pub fn prepare_remapped_jar(temp_dir: &Path) -> PathBuf {
    let remapped_jar = temp_dir.join(format!("server-{}-remapped.jar", MC_VERSION));

    if remapped_jar.exists() {
        println!("Found cached remapped JAR.");
        return remapped_jar;
    }

    println!("Remapped JAR not found. Starting Decompilation process...");

    // 1. Download Decompiler
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

    // 2. Run Decompiler
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
    println!("Compiling BlockInfoDumper...");

    // 1. Write Java Source
    let java_file = temp_dir.join("BlockInfoDumper.java");
    fs::write(&java_file, JAVA_DUMPER_SOURCE).unwrap();

    // 2. Build Classpath
    // Use absolute paths to avoid confusion
    let remapped_abs = remapped_jar.canonicalize().unwrap();
    let cp_sep = if cfg!(windows) { ";" } else { ":" };
    let mut classpath = format!(".{}{}", cp_sep, remapped_abs.display());

    let lib_dir = temp_dir.join("libraries");
    let mut jar_count = 0;
    let mut found_dfu = false;

    if lib_dir.exists() {
        for entry in WalkDir::new(&lib_dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "jar") {
                // Get absolute path
                if let Ok(abs_path) = path.canonicalize() {
                    classpath.push_str(cp_sep);
                    classpath.push_str(&abs_path.to_string_lossy());

                    jar_count += 1;
                    let fname = path.file_name().unwrap().to_string_lossy();
                    if fname.contains("datafixerupper") {
                        found_dfu = true;
                    }
                }
            }
        }
    }

    println!("    Found {} libraries.", jar_count);
    if !found_dfu {
        println!(
            "WARNING: 'datafixerupper' JAR not found in libraries! Compilation will likely fail."
        );
        println!("Check contents of: {}", lib_dir.display());
    }

    // 3. Compile
    let compile_status = Command::new("javac")
        .current_dir(temp_dir)
        .arg("-cp")
        .arg(&classpath)
        .arg("BlockInfoDumper.java")
        .output()
        .expect("Failed to run javac");

    if !compile_status.status.success() {
        panic!(
            "Java compilation failed:\n{}",
            String::from_utf8_lossy(&compile_status.stderr)
        );
    }

    println!("Running BlockInfoDumper...");

    // 4. Run
    let json_path = temp_dir.join("physics.json");
    // Use absolute path for output arg so Java can find it easily
    // (Use current dir for output since json_path is not created yet)
    let json_abs_path = temp_dir.canonicalize().unwrap().join("physics.json");

    let output = Command::new("java")
        .current_dir(temp_dir)
        .arg("-cp")
        .arg(&classpath)
        .arg("BlockInfoDumper")
        .arg(&json_abs_path)
        .output()
        .expect("Failed to run Java extractor");

    if !output.status.success() {
        // --- Print full logs before panicking ---
        println!("Java execution failed!");
        println!(
            "--- STDOUT ---\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        println!(
            "--- STDERR ---\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("Extractor failed (see logs above)");
    }

    if !json_path.exists() {
        panic!(
            "Java finished but physics.json was not created at {:?}",
            json_path
        );
    }

    println!("Physics data extracted successfully.");
    json_path
}
