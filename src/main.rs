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
use fs_extra;
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

/// mmmm maybe later wrap
impl Error for LutwigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

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
fn patch(cache: PathBuf, target: PathBuf) -> Result<(), Box<dyn Error>> {
    if !target.is_dir() {
        return Err(Box::new(LutwigError::InvalidPatchTarget));
    }

    static MIRROR: &str = "https://archive.org/download/vxacertp.tar/vxacertp.tar.gz";

    let patches = [
        "Audio/BGM",
        "Audio/BGS",
        "Audio/ME",
        "Audio/SE",
        "Fonts",
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

    let vxlib = cache.join("vxacertp/RPGVXAce");
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

    println!("Applying patches...");

    for patch in patches.iter() {
        let patch_lib = vxlib.join(patch);
        let patch_target = target.join(patch);

        if !patch_target.exists() {
            fs::create_dir_all(&patch_target)?;
        }

        let entries: Vec<PathBuf> = fs::read_dir(patch_lib)?
            .filter_map(|entry| entry.ok().and_then(|e| Some(e.path())))
            .collect();

        fs_extra::copy_items(
            entries.as_slice(),
            &patch_target,
            &fs_extra::dir::CopyOptions::default(),
        );

        println!("{}", patch_target.display());
    }

    println!("Patches applied successfully.");

    Ok(())
}

/// Locates your Steam path and adds the target game directory into your library.
fn install(cache: PathBuf, target: PathBuf) -> Result<(), Box<dyn Error>> {
    if !target.is_dir() {
        return Err(Box::new(LutwigError::InvalidInstallTarget));
    }

    Ok(())
}

//fn uninstall() {}

#[derive(Subcommand)]
enum Commands {
    Patch { patch_target: PathBuf },
    Install { install_target: PathBuf },
    Uninstall { uninstall_target: String },
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, value_name = "DIR", global = true)]
    cache: Option<PathBuf>,
    #[arg(short, long, global = true)]
    cleanup: Option<bool>,
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
