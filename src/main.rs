mod template;
mod volunteers;

use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::Parser;

use volunteers::Volunteer;

#[derive(Debug, clap::Parser)]
pub struct Config {
    /// The directory containing all of the volunteer description toml files.
    #[arg(long, short)]
    volunteer_dir: String,

    /// The directory in which the pictures of the volunteers are placed
    #[arg(long, short)]
    picture_dir: String,

    /// The directory containt the template including all assets like fonts and
    /// packages
    #[arg(long, short)]
    template_dir: String,

    /// The output file name under which the generated PDF is save to.
    #[arg(long, short)]
    output: Option<String>,
}

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    let config = Config::parse();
    let template_compiler = template::TemplateCompiler::from_config(&config)
        .context("could not build template compiler from configuration")?;

    let picture_dir = PathBuf::from(&config.picture_dir);
    if !picture_dir.is_dir() {
        anyhow::bail!("the picture directory does not exists");
    }

    let volunteers = load_volunteers_from_dir(&config.volunteer_dir, &picture_dir)?
        .into_iter()
        .map(Into::into)
        .collect();
    let template_inputs = template::TemplateInputs::new(volunteers);

    let pdf_bytes = template_compiler.generate_pdf(template_inputs)?;
    let output_file = match config.output {
        Some(o) => o,
        None => {
            let current_time = chrono::Local::now();
            current_time.format("%FT%H-%M-%S_Stempelkarten.pdf").to_string()
        }
    };
    std::fs::write(&output_file, pdf_bytes).context("could not write to output file")?;

    Ok(())
}

/// Try to load all volunteer files from the given directory.
///
/// All files that end on `.toml` within or in a subdirectory are considered as
/// volunteer files. If any of the files is invalid, an error is returned.
fn load_volunteers_from_dir(dir: &str, picture_dir: &Path) -> anyhow::Result<Vec<Volunteer>> {
    let input_path = validate_volunteer_directory(dir)?;
    let search_path = build_volunteer_search_path(&input_path);

    let glob_search_term = search_path.to_str().ok_or(anyhow::anyhow!(
        "the volunteer directory contains invalid characters"
    ))?;

    let glob_search: glob::Paths =
        glob::glob(glob_search_term).context("the constructed search term is invalid")?;

    let mut volunteers = vec![];
    for volunteer_file in glob_search {
        match volunteer_file {
            Ok(volunteer_file) => {
                log::info!(
                    "found volunteer description file '{}'",
                    volunteer_file.display()
                );

                let volunteer = Volunteer::from_file(&volunteer_file)?;
                volunteer.ensure_picture_exists(picture_dir)?;
                volunteers.push(volunteer);
            }
            Err(e) => {
                log::warn!("volunteer directory search run into an error: {e}")
            }
        }
    }

    Ok(volunteers)
}

/// Ensure the volunteer input directory actually exists. If the directory
/// exists, the absolute path is returned.
pub fn validate_volunteer_directory(dir: &str) -> anyhow::Result<PathBuf> {
    log::debug!("validating the input directory '{dir}' is valid");
    let path = std::path::absolute(dir)
        .context("could not construct absolute path from the given input directory")?;

    log::debug!(
        "ensuring the file system contains the input directory '{}'",
        path.display()
    );

    if !path.exists() {
        anyhow::bail!(
            "the file system does not contain the input directory '{}'",
            path.display()
        );
    }

    if path.is_file() {
        anyhow::bail!("the provided directory must not be a file");
    }

    Ok(path)
}

/// Build the search path for volunteer files using the glob syntax. The seach
/// path recurses through the subdirectories.
pub fn build_volunteer_search_path(dir: &Path) -> PathBuf {
    let mut search_path = std::path::PathBuf::from(dir);
    search_path.push("**/*.toml");
    search_path
}
