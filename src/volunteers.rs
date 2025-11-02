use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Volunteer {
    pub first_name: String,
    pub last_name: String,
    pub barcode: String,
    pub qualified: bool,
    pub picture: String,
    pub deployment: Vec<String>,
    pub licenses: Vec<String>,
    pub qualifications: Vec<String>,
    pub hide_indicator: Option<bool>,
}

impl Volunteer {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        log::debug!("loading volunteer from file '{}'", path.display());
        Self::ensure_volunteer_file_exists(path)?;

        let file_contents =
            std::fs::read_to_string(path).context("could not read volunteer description file")?;

        let volunteer = match toml::from_str::<Volunteer>(&file_contents) {
            Ok(volunteer) => volunteer,
            Err(e) => {
                anyhow::bail!(
                    "invalid volunteer description file '{}'\n{}",
                    path.display(),
                    e
                )
            }
        };

        Ok(volunteer)
    }

    fn ensure_volunteer_file_exists(volunteer_path: &Path) -> anyhow::Result<()> {
        if !volunteer_path.exists() {
            anyhow::bail!("volunteer description path does not exist");
        }

        if !volunteer_path.is_file() {
            anyhow::bail!("volunteer description path is not a file");
        }

        Ok(())
    }

    pub fn ensure_picture_exists(&self, picture_dir: &Path) -> anyhow::Result<()> {
        let mut picture_path = PathBuf::from(picture_dir);
        picture_path.push(&self.picture);

        if !picture_path.exists() {
            anyhow::bail!(
                "the volunteer picture path does not exist '{}'",
                picture_path.display()
            );
        }

        if !picture_path.is_file() {
            anyhow::bail!(
                "the volunteer picture is not a file '{}'",
                picture_path.display()
            );
        }

        Ok(())
    }
}
