use std::{
    error::Error,
    fmt::{self, Display},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::exit,
};

use clap::{Parser, Subcommand};
use dirs;
use reqwest;

/// Error enum for anything that doesn't handle their own.
#[derive(Clone, Copy, Debug)]
enum LutwigError {
    InvalidCache,
    InvalidHome,
    InvalidPatchTarget,
    InvalidInstallTarget,
}

impl Display for LutwigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LutwigError::InvalidCache => "Cache path is invalid.",
                LutwigError::InvalidHome => "Home path set is invalid.",
                LutwigError::InvalidPatchTarget => "Patch target directory is invalid.",
                LutwigError::InvalidInstallTarget => "Install target directory is invalid.",
            }
        )
    }
}

impl Error for LutwigError {}

/// Locates and creates .cache/lutwig if it doesn't exist.
fn setup(cache_local: Option<PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    let cache = {
        if let Some(_cache) = cache_local.or(dirs::cache_dir()) {
            _cache.join("lutwig")
        } else {
            return Err(Box::new(LutwigError::InvalidHome));
        }
    };

    if !cache.exists() {
        match fs::create_dir_all(&cache) {
            Ok(_) => {
                if let Some(home) = dirs::home_dir() {
                    let meta = home.join(".lwcache");
                    let mut file = fs::File::create(&meta)?;
                    file.write(meta.into_os_string().into_string().unwrap().as_bytes())?;
                } else {
                    return Err(Box::new(LutwigError::InvalidCache));
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    };

    Ok(cache)
}

fn patch(cache: PathBuf, path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        return Err(Box::new(LutwigError::InvalidPatchTarget));
    }

    static MIRROR: &str = "https://dl.komodo.jp/rpgmakerweb/run-time-packages/RPGVXAce_RTP.zip";

    let audio_patch = ["BGM", "BGS", "ME", "SE"];

    let graphics_patch = [
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

    match reqwest::blocking::get(MIRROR) {
        Ok(mut resp) => {
            if resp.status().is_success() {
                let lib = cache.join("vxacerpt");
                match fs::File::create(lib) {
                    Ok(mut file) => {
                        io::copy(&mut resp, &mut file).unwrap();
                    }
                    Err(e) => {
                        println!("Patching data failed to be created locally: {}", e);
                        exit(1);
                    }
                }
            }
        }
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}

fn install(cache: PathBuf, path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        return Err(Box::new(LutwigError::InvalidInstallTarget));
    }

    Ok(())
}

//fn library() {}

#[derive(Subcommand)]
enum Commands {
    Patch {
        #[arg(short, long, value_name = "DIR")]
        patch_target: PathBuf,
    },
    Install {
        #[arg(short, long, value_name = "DIR")]
        install_target: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, value_name = "DIR", global = true)]
    cache: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let cache = setup(cli.cache)?;

    match cli.command {
        Commands::Patch { patch_target } => {
            patch(cache, patch_target)?;
        }
        Commands::Install { install_target } => {
            install(cache, install_target)?;
        }
    };

    Ok(())
}
