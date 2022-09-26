use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
struct ArnoldVersion {
    arch: u16,
    major: u16,
    minor: u16,
    fix: String,
}

const CRATE_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const CRATE_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const CRATE_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

/// Parse the arnold header for version number
///
/// We are looking for the following pattern:
/// #define AI_VERSION_ARCH_NUM    7
/// #define AI_VERSION_MAJOR_NUM   1
/// #define AI_VERSION_MINOR_NUM   1
/// #define AI_VERSION_FIX         "0"
///
fn parse_arnold_version(
    version_file: &PathBuf,
) -> Result<ArnoldVersion, Box<dyn std::error::Error>> {
    let file = File::open(version_file)?;

    let mut arch: Option<u16> = None;
    let mut major: Option<u16> = None;
    let mut minor: Option<u16> = None;
    let mut fix: Option<String> = None;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.starts_with("#define AI_VERSION_ARCH_NUM") {
                arch = Some(u16::from_str_radix(&line[28..].trim(), 10).unwrap());
            }
            if line.starts_with("#define AI_VERSION_MAJOR_NUM") {
                major = Some(u16::from_str_radix(&line[29..].trim(), 10).unwrap());
            }
            if line.starts_with("#define AI_VERSION_MINOR_NUM") {
                minor = Some(u16::from_str_radix(&line[29..].trim(), 10).unwrap());
            }
            if line.starts_with("#define AI_VERSION_FIX") {
                fix = Some(line[23..].trim().to_owned());
                break;
            }
        }
    }
    Ok(ArnoldVersion {
        arch: arch.unwrap(),
        major: major.unwrap(),
        minor: minor.unwrap(),
        fix: fix.unwrap(),
    })
}

fn main() {
    let arnold_path = PathBuf::from(env::var("ARNOLD_ROOT").expect("ARNOLD_ROOT is set"));
    let arnold_include_dir = arnold_path.join("include");
    let arnold_lib_dir = arnold_path.join("bin");
    let version = parse_arnold_version(&arnold_include_dir.join("ai_version.h"))
        .expect("can get arnold version");

    let crate_arch = u16::from_str_radix(&CRATE_VERSION_MAJOR, 10).expect("crate arch is a number");
    let crate_major =
        u16::from_str_radix(&CRATE_VERSION_MINOR, 10).expect("crate major is a number");
    let crate_minor =
        u16::from_str_radix(&CRATE_VERSION_PATCH, 10).expect("crate minor is a number");

    assert_eq!(crate_arch, version.arch, "arch versions must be the same");
    if version.major > crate_major {
        panic!("arnold major version can't be higher than crate major version")
    }
    if version.major == crate_major && version.minor > crate_minor {
        panic!("arnold minor version can't be higher than crate minor version")
    }

    // -----------------------------

    println!("cargo:rustc-link-search={}", &arnold_lib_dir.display());
    println!("cargo:rustc-link-lib=ai");
}
