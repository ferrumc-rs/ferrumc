use std::collections::HashMap;
use std::env::consts::OS;
use std::io::Cursor;
use std::process::{exit, Stdio};

use simdnbt::owned::Nbt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{error, warn};

use ferrumc_utils::config::get_global_config;
use ferrumc_utils::error::Error;

pub async fn start_database() -> Result<(), Error> {
    let envs: HashMap<&str, String> = HashMap::from([
        (
            "SURREAL_BIND",
            format!(
                "127.0.0.1:{}",
                get_global_config().database.port.to_string()
            ),
        ),
        (
            "SURREAL_PATH",
            format!("file:{}", get_global_config().database.path),
        ),
        ("SURREAL_LOG_LEVEL", "info".to_string()),
        ("SURREAL_NO_BANNER", "true".to_string()),
    ]);

    let mut executable_name = match OS {
        "windows" => ".\\surreal.exe",
        "macos" => "./surreal",
        "linux" => "./surreal",
        _ => {
            return Err(Error::Generic("Unsupported OS".to_string()));
        }
    };

    if !tokio::fs::try_exists(executable_name).await.unwrap() {
        if which::which("surreal").is_ok() {
            warn!("Using system surreal executable. This may be outdated.");
            executable_name = "surreal";
        } else {
            error!("Surreal executable not found at {}", executable_name);
            exit(1);
        }
    }

    let mut surreal_setup_process = tokio::process::Command::new(executable_name)
        .arg("start")
        .envs(envs)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Why the fuck does surrealdb use stderr by default???
    let mut stderr = BufReader::new(surreal_setup_process.stderr.take().unwrap());
    let mut output = String::new();
    loop {
        stderr.read_line(&mut output).await.unwrap();
        if !output.is_empty() {
            print!("SURREAL: {}", output);
            output.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::load_chunk;

    #[tokio::test]
    async fn test_load_chunks() {
        assert!(load_chunk(0, 0).await.is_ok());
        assert!(load_chunk(-5000, 4025).await.is_err());
        assert_eq!(
            load_chunk(0, 0)
                .await
                .unwrap()
                .data
                .unwrap()
                .get("yPos")
                .unwrap()
                .int()
                .unwrap(),
            -4
        );
    }
}

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub data: Nbt,
}

pub async fn load_chunk(x: i32, z: i32) -> Result<Chunk, Error> {
    // TODO: Replace with database call when that is all set up
    let region_area = (
        (x as f64 / 32.0).floor() as i32,
        (z as f64 / 32.0).floor() as i32,
    );
    let region_file = std::fs::File::open("dummyregion.mca")?;
    let mut region = fastanvil::Region::from_stream(region_file).unwrap();
    let raw_chunk_data = region
        .read_chunk(x as usize, z as usize)
        .map_err(|e| {
            Error::Generic(format!(
                "Unable to read chunk {} {} from region {} {} ",
                x, z, region_area.0, region_area.1
            ))
        })?
        .expect(
            format!(
                "Chunk {} {} not found in region {} {}",
                x, z, region_area.0, region_area.1
            )
            .as_str(),
        );
    let decoded_chunk = simdnbt::owned::read(&mut Cursor::new(&*raw_chunk_data)).unwrap();
    Ok(Chunk {
        x,
        z,
        data: decoded_chunk,
    })
}
