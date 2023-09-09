use std::{
    collections::HashMap,
    env::{self, VarError},
    fs,
    path::{Path, PathBuf},
    process::{self, exit},
};

use clap::{arg, command, value_parser, ArgAction, Command};

/// Validates $HOME directory and locates/creates .cache/lutwig.
fn setup<P: AsRef<Path>>(cache_local: Option<P>) -> (PathBuf, PathBuf) {
    assert_eq!(env::consts::OS, "linux", "Target system is not Linux.");

    let home: PathBuf = match env::var("HOME") {
        Ok(_path) => {
            let path = Path::new(_path.as_str());
            if path.exists() && path.is_dir() {
                path.to_path_buf()
            } else {
                println!("$HOME points to an invalid location.");
                exit(1);
            }
        }
        Err(e) => {
            println!("$HOME is not defined.");
            exit(1);
        }
    };

    let cache: PathBuf = {
        let _cache = {
            if let Some(_path) = cache_local {
                let path = _path.as_ref().to_path_buf();
                if path.exists() && path.is_dir() && path.is_absolute() {
                    path
                } else {
                    println!("Supplied custom cache path is not valid.");
                    exit(1);
                }
            } else {
                home.join(".cache")
            }
        };
        _cache.join("lutwig")
    };

    if !cache.exists() {
        match fs::create_dir(&cache) {
            Err(_) => {
                println!("Failed creating cache directory: {}", cache.display());
            }
            _ => {}
        }
    };

    (home, cache)
}

fn main() {
    let (home, cache) = setup::<&str>(None);

    let mirrors: HashMap<_, [_; 1]> = HashMap::from([
        (
            "blacksouls", [
                "https://bafybeif3iffn6g2m2clvhr7ywqh4o7nvtjife2ry5s3azyvxplkftgnbem.ipfs.dweb.link/DL/RJ237469%20-%20Black%20Souls%20II.7z"   
            ],
        ),
        (
            "blacksouls2", [
                "https://bafybeif3iffn6g2m2clvhr7ywqh4o7nvtjife2ry5s3azyvxplkftgnbem.ipfs.dweb.link/DL/RE251629%20-%20BLACK%20SOULS.7z"
            ],
        ),
    ]);

    let vx_mirrors = ["https://dl.komodo.jp/rpgmakerweb/run-time-packages/RPGVXAce_RTP.zip"];

    let audio_override = ["BGM", "BGS", "ME", "SE"];

    let graphics_override = [
        "Animations",
        "Battlebacks1",
        "Battlebacks2",
        "Battlers",
        "Characters",
        "Faces",
        "Parallaxes",
        "System",
        "Tilesets",
        "Titles1",
        "Titles2",
    ];
}
