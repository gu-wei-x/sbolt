use std::{
    collections::BTreeMap,
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

use serde::Deserialize;

#[derive(Deserialize)]
struct Metadata {
    target_directory: PathBuf,
    workspace_root: PathBuf,
    workspace_members: Vec<String>,
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    id: String,
    name: String,
    manifest_path: PathBuf,
    targets: Vec<Target>,
    default_run: Option<String>,
    metadata: Option<PackageMetadata>,
}

#[derive(Deserialize)]
struct PackageMetadata {
    bundle: Option<BundleConfig>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct BundleConfig {
    #[serde(default)]
    assets: BTreeMap<PathBuf, PathBuf>,
    #[serde(default)]
    files: BTreeMap<PathBuf, PathBuf>,
}

#[derive(Deserialize)]
#[serde(tag = "reason")]
enum BuildMessage {
    #[serde(rename = "compiler-artifact")]
    Artifact {
        target: Target,
        executable: Option<PathBuf>,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
}

#[derive(Default)]
struct Options {
    package: Option<String>,
    binary: Option<String>,
    profile: Option<String>,
}

fn options(args: impl IntoIterator<Item = OsString>) -> Result<Options, Box<dyn Error>> {
    let mut options = Options::default();
    let mut args = args.into_iter();
    while let Some(option) = args.next() {
        let option = option.to_str().ok_or("Options must be valid UTF-8.")?;
        let field = match option {
            "--package" | "-p" => &mut options.package,
            "--bin" => &mut options.binary,
            "--Profile" | "-Profile" | "--profile" => &mut options.profile,
            _ => return Err(format!("Unknown option: {option}").into()),
        };
        let value = args
            .next()
            .ok_or_else(|| format!("Missing value for {option}."))?;
        let value = value
            .into_string()
            .map_err(|_| "Option values must be valid UTF-8.")?;
        if value.is_empty() || value.starts_with('-') {
            return Err(format!("Invalid value for {option}: {value:?}").into());
        }
        if field.replace(value).is_some() {
            return Err(format!("Duplicate option: {option}").into());
        }
    }
    if let Some(profile) = &mut options.profile {
        profile.make_ascii_lowercase();
        if profile != "release" && profile != "debug" {
            return Err("Invalid profile: expected release or debug.".into());
        }
    }
    Ok(options)
}

fn cargo_output(project: &Path, args: &[&str]) -> Result<Vec<u8>, Box<dyn Error>> {
    let output = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(project)
        .args(args)
        .stderr(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        return Err(format!("Cargo {} failed: {}", args[0], output.status).into());
    }
    Ok(output.stdout)
}

fn copy_assets(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_assets(&entry.path(), &target)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), target)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("Unsupported asset type: {}", entry.path().display()),
            ));
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = options(env::args_os().skip(1))?;
    let project = env::current_dir()?;
    let metadata: Metadata = serde_json::from_slice(&cargo_output(
        &project,
        &["metadata", "--no-deps", "--format-version", "1"],
    )?)?;
    let candidates: Vec<_> = metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .filter(|package| match &options.package {
            Some(name) => package.name == *name,
            None => package
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.bundle.as_ref())
                .is_some(),
        })
        .collect();
    let package = match candidates.as_slice() {
        [package] => *package,
        [] => return Err("No matching workspace package. Specify --package <name> or configure [package.metadata.bundle].".into()),
        _ => return Err("Multiple packages have bundle settings. Specify --package <name>.".into()),
    };
    let binaries: Vec<_> = package
        .targets
        .iter()
        .filter(|target| target.kind.iter().any(|kind| kind == "bin"))
        .collect();
    let binary = match options.binary.as_ref().or(package.default_run.as_ref()) {
        Some(name) => binaries
            .iter()
            .find(|target| target.name == *name)
            .copied()
            .ok_or_else(|| format!("Package {} has no binary named {name}.", package.name))?,
        None => match binaries.as_slice() {
            [binary] => *binary,
            [] => return Err(format!("Package {} has no binaries.", package.name).into()),
            _ => return Err("Multiple binaries available. Specify --bin <name>.".into()),
        },
    };
    let project = package
        .manifest_path
        .parent()
        .ok_or("Manifest has no parent directory.")?;
    let config = package
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.bundle.as_ref());
    if let Some(config) = config {
        for (source, destination) in config.assets.iter().chain(&config.files) {
            if source.is_absolute()
                || source
                    .components()
                    .any(|part| matches!(part, Component::Prefix(_) | Component::RootDir))
            {
                return Err(format!(
                    "Bundle source must be relative to the package: {}",
                    source.display()
                )
                .into());
            }
            if destination.as_os_str().is_empty()
                || destination
                    .components()
                    .any(|part| !matches!(part, Component::Normal(_)))
            {
                return Err(format!(
                    "Bundle destination must stay inside the bundle: {}",
                    destination.display()
                )
                .into());
            }
        }
    }
    let profile = options.profile.as_deref().unwrap_or("release");
    let build = cargo_output(
        &metadata.workspace_root,
        &[
            "build",
            "--profile",
            if profile == "debug" { "dev" } else { "release" },
            "--package",
            &package.name,
            "--bin",
            &binary.name,
            "--message-format=json-render-diagnostics",
        ],
    )?;
    let mut executables = Vec::new();
    for line in io::BufRead::lines(io::Cursor::new(build)) {
        if let BuildMessage::Artifact {
            target,
            executable: Some(path),
        } = serde_json::from_str(&line?)?
            && target.name == binary.name
        {
            executables.push(path);
        }
    }
    let [executable] = executables.as_slice() else {
        return Err(format!(
            "Cargo did not produce exactly one {} executable.",
            binary.name
        )
        .into());
    };
    let bundle = metadata
        .target_directory
        .join("bundle")
        .join(&package.name)
        .join(profile);
    let name = executable
        .file_name()
        .ok_or("Executable has no file name.")?;
    let executable_name = name
        .to_str()
        .ok_or("Executable name must be valid UTF-8.")?;
    let (shell, command) = if cfg!(windows) {
        ("powershell", format!(".\\{executable_name}"))
    } else {
        ("sh", format!("./{executable_name}"))
    };
    if bundle.try_exists()? {
        fs::remove_dir_all(&bundle)?;
    }
    fs::create_dir_all(&bundle)?;
    fs::copy(executable, bundle.join(name))?;
    fs::write(
        bundle.join("README.md"),
        format!(
            "# {}\n\nRun from this directory:\n\n```{shell}\n{command}\n```\n\nKeep bundled assets beside the executable.\n",
            package.name
        ),
    )?;
    if let Some(config) = config {
        for (source, destination) in &config.assets {
            copy_assets(&project.join(source), &bundle.join(destination))
                .map_err(|error| format!("Cannot copy assets {}: {error}", source.display()))?;
        }
        for (source, destination) in &config.files {
            let destination = bundle.join(destination);
            fs::create_dir_all(
                destination
                    .parent()
                    .ok_or("Destination has no parent directory.")?,
            )?;
            fs::copy(project.join(source), &destination)
                .map_err(|error| format!("Cannot copy file {}: {error}", source.display()))?;
        }
    }
    println!("Deployment bundle: {}", bundle.display());
    Ok(())
}
