use std::{
    collections::HashMap,
    env::{self, VarError},
    path::{Path, PathBuf},
    process::{self, exit},
};

use clap::{arg, command, value_parser, ArgAction, Command};

/// Validates $HOME directory and locates/creates .cache/lutwig.
fn setup() {}

fn main() {
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
