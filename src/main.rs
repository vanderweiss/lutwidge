#![allow(unused)]

use std::{
    error::Error,
    fmt::{self, Display},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
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
    InvalidMirrorRequest,
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
                LutwigError::InvalidMirrorRequest => "Mirror requested is not operational. Try again later or file an issue if it persists.",
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

/// Merges assets and dependencies from the library with the target game directory.
fn patch(cache: PathBuf, path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.is_dir() {
        return Err(Box::new(LutwigError::InvalidPatchTarget));
    }

    static MIRROR: &str = "https://archive.org/download/vxacertp.tar/vxacertp.tar.gz";

    let audio_patch = ["Audio/BGM", "Audio/BGS", "Audio/ME", "Audio/SE"];

    let graphics_patch = [
        "Graphics/Animations",
        "Graphics/Battlebacks1",
        "Graphics/Battlebacks2",
        "Graphics/Battlers",
        "Graphics/Characters",
        "Graphics/Faces",
        "Graphics/Parallaxes",
        "Graphics/System",
        "Graphics/Tilesets",
        "Graphics/Titles1",
        "Graphics/Titles2",
    ];

    let vxlib = cache.join("vxacertp");
    let vxlib_tar = vxlib.with_extension("tar.gz");

    println!("Download starting...");

    if !vxlib_tar.exists() && !vxlib.exists() {
        let mut resp = reqwest::blocking::get(MIRROR)?;
        if resp.status().is_success() {
            let mut file = fs::File::create(&vxlib_tar)?;
            io::copy(&mut resp, &mut file)?;
            println!("Download complete!")
        } else {
            return Err(Box::new(LutwigError::InvalidMirrorRequest));
        }
    } else {
        println!("Previous download found!")
    }

    println!("Unpacking download...");

    if !vxlib.exists() {
        Command::new("tar")
            .arg("-xzf")
            .arg(vxlib_tar.as_os_str())
            .arg("--directory")
            .arg(cache.as_os_str())
            .spawn()?;
        println!("Unpacked download!");
    } else {
        println!("Download already unpacked!");
    }

    for _patch in audio_patch.iter() {
        let patch = vxlib.join(_patch);
        for entry in fs::read_dir(patch)? {}
    }

    Ok(())
}

/// Locates your Steam path and adds the target game directory into your library.
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
    Uninstall {
        #[arg(short, long, value_name = "NAME")]
        uninstall_target: String,
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
        Commands::Uninstall { uninstall_target } => {}
    };

    Ok(())
}
