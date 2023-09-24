#![allow(unused)]

use std::{
    error::Error,
    fmt::{self, Display},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    result,
};

use clap::{Parser, Subcommand};
use dirs;
use fs_extra;
use reqwest;

// Custom error for nicer outputs
#[derive(Debug)]
enum LwError {
    Io(io::Error),
    Mirror(reqwest::Error),
}

impl Display for LwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LwError::Io(ref err) => writeln!(f, "IO error: {}", err),
            LwError::Mirror(ref err) => writeln!(f, "Mirror request error: {}", err),
        }
    }
}

impl From<io::Error> for LwError {
    fn from(error: io::Error) -> Self {
        LwError::Io(error)
    }
}

impl From<reqwest::Error> for LwError {
    fn from(error: reqwest::Error) -> Self {
        LwError::Mirror(error)
    }
}

type Result<T> = result::Result<T, LwError>;

/// Locates and creates .cache/lutwig if it doesn't exist.
fn setup(cache_local: Option<PathBuf>) -> Result<PathBuf> {
    let home = dirs::home_dir().expect("Home directory is not set up correctly.");
    let cache = cache_local.unwrap_or(home.join(".cache/lutwig"));

    if !cache.exists() {
        fs::create_dir_all(&cache)?;
    }

    let meta = home.join(".lwcache");
    let mut handler = fs::File::create(&meta)?;
    handler.write(meta.into_os_string().into_string().unwrap().as_bytes());

    Ok(cache)
}

/// Merges assets and dependencies from the library with the target game directory.
fn patch(cache: PathBuf, target: PathBuf) -> Result<()> {
    const MIRROR: &str = "https://archive.org/download/vxacertp.tar/vxacertp.tar.gz";

    const PATCHES: [&str; 16] = [
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

    let vx = cache.join("vxacertp/RPGVXAce");
    let vxt = vx.with_extension("tar.gz");

    println!("- [1/3] Download starting...");

    if !vxt.exists() {
        let mut resp = reqwest::blocking::get(MIRROR)?;
        let mut handler = fs::File::create(&vxt)?;
        io::copy(&mut resp, &mut handler)?;
    }

    println!("- [2/3] Unpacking download...");

    if !vx.exists() {
        Command::new("tar")
            .arg("-xzf")
            .arg(vxt.as_os_str())
            .arg("--directory")
            .arg(cache.as_os_str())
            .spawn()?;
    }

    println!("- [3/3] Applying patches...");

    for patch in PATCHES.iter() {
        let patch_lib = vx.join(patch);
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

    println!("- DONE!");

    Ok(())
}

/// Locates your Steam path and adds the target game directory into your library.
fn install(cache: PathBuf, target: PathBuf) -> Result<()> {
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
    #[arg(short = 'C')]
    #[arg(long, value_name = "DIR", global = true)]
    cache: Option<PathBuf>,
    #[arg(short, long, global = true)]
    cleanup: Option<bool>,
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cache = setup(cli.cache)?;

    match cli.command {
        Commands::Patch { patch_target } => {
            patch(cache, patch_target);
        }
        Commands::Install { install_target } => {
            install(cache, install_target)?;
        }
        Commands::Uninstall { uninstall_target } => {}
    };

    Ok(())
}
