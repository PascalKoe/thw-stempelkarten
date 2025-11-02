use std::path::PathBuf;

use anyhow::{Context, Result};
use typst::foundations::{Array, Dict, Str, Value};
use typst_as_lib::{
    TypstEngine, cached_file_resolver::IntoCachedFileResolver, file_resolver::FileSystemResolver,
    typst_kit_options::TypstKitFontOptions,
};

use crate::{Config, volunteers};

pub struct TemplateCompiler {
    typst_engine: TypstEngine,
}

impl TemplateCompiler {
    pub fn from_config(config: &Config) -> Result<Self> {
        let file_resolver =
            Self::build_file_resolver(config).context("could not build template file resolver")?;
        let fonts_resolver =
            Self::build_fonts_resolver(config).context("could not build font resolver")?;

        let mut template_file = PathBuf::from(&config.template_dir);
        template_file.push("template.typ");
        if !template_file.is_file() {
            anyhow::bail!("the template directory does not contain a 'template.typ' file");
        }

        let typst_engine = typst_as_lib::TypstEngine::builder()
            .add_file_resolver(file_resolver.into_cached())
            .with_file_system_resolver(&config.picture_dir)
            .search_fonts_with(fonts_resolver)
            .build();

        Ok(Self { typst_engine })
    }

    fn build_file_resolver(config: &Config) -> Result<FileSystemResolver> {
        let template_path = PathBuf::from(&config.template_dir);
        if !template_path.is_dir() {
            anyhow::bail!("the template directory is not a valid directory or does not exist");
        }

        let mut package_path = template_path.clone();
        package_path.push("packages");
        if !package_path.is_dir() {
            anyhow::bail!("the template directory does not contain a 'packages' subdirectory");
        }

        Ok(FileSystemResolver::new(template_path).local_package_root(package_path))
    }

    fn build_fonts_resolver(config: &Config) -> Result<TypstKitFontOptions> {
        let mut fonts_path = PathBuf::from(&config.template_dir);
        fonts_path.push("fonts");
        if !fonts_path.is_dir() {
            anyhow::bail!("the template directory does not contain a 'fonts' subdirectory");
        }

        Ok(TypstKitFontOptions::new().include_dirs([fonts_path]))
    }

    pub fn generate_pdf(&self, template_inputs: TemplateInputs) -> Result<Vec<u8>> {
        let document = self
            .typst_engine
            .compile_with_input("template.typ", template_inputs)
            .output
            .context("document generation failed with the provided input data")?;

        let pdf_document = typst_pdf::pdf(&document, &Default::default())
            .expect("could not generate the PDF from the generated document");

        Ok(pdf_document)
    }
}

pub struct TemplateInputs {
    volunteers: Array,
}

impl TemplateInputs {
    pub fn new(volunteers: Vec<TemplateVolunteer>) -> Self {
        let volunteers = Array::from_iter(
            volunteers
                .into_iter()
                .map(TemplateVolunteer::into)
                .map(Value::Dict),
        );
        Self { volunteers }
    }
}

impl From<TemplateInputs> for Dict {
    fn from(inputs: TemplateInputs) -> Self {
        let mut dict = Dict::new();
        dict.insert(Str::from("volunteers"), Value::Array(inputs.volunteers));
        dict
    }
}

#[derive(Debug, Clone)]
pub struct TemplateVolunteer {
    pub first_name: Str,
    pub last_name: Str,
    pub barcode: Str,
    pub qualified: bool,
    pub hide_qualified: bool,
    pub picture: Str,
    pub deployment: Array,
    pub licenses: Array,
    pub qualifications: Array,
}

impl From<TemplateVolunteer> for Dict {
    fn from(volunteer: TemplateVolunteer) -> Self {
        let mut dict = Dict::new();

        dict.insert(Str::from("first_name"), Value::Str(volunteer.first_name));
        dict.insert(Str::from("last_name"), Value::Str(volunteer.last_name));
        dict.insert(Str::from("barcode"), Value::Str(volunteer.barcode));
        dict.insert(Str::from("qualified"), Value::Bool(volunteer.qualified));
        dict.insert(Str::from("picture"), Value::Str(volunteer.picture));
        dict.insert(Str::from("deployment"), Value::Array(volunteer.deployment));
        dict.insert(Str::from("licenses"), Value::Array(volunteer.licenses));
        dict.insert(
            Str::from("hide_qualified"),
            Value::Bool(volunteer.hide_qualified),
        );
        dict.insert(
            Str::from("qualifications"),
            Value::Array(volunteer.qualifications),
        );

        dict
    }
}

impl From<volunteers::Volunteer> for TemplateVolunteer {
    fn from(volunteer: volunteers::Volunteer) -> Self {
        let deployment = Array::from_iter(
            volunteer
                .deployment
                .into_iter()
                .map(Str::from)
                .map(Value::Str),
        );

        let licenses = Array::from_iter(
            volunteer
                .licenses
                .into_iter()
                .map(Str::from)
                .map(Value::Str),
        );

        let qualifications = Array::from_iter(
            volunteer
                .qualifications
                .into_iter()
                .map(Str::from)
                .map(Value::Str),
        );

        Self {
            first_name: Str::from(volunteer.first_name),
            last_name: Str::from(volunteer.last_name),
            barcode: Str::from(volunteer.barcode),
            qualified: volunteer.qualified,
            picture: Str::from(volunteer.picture),
            deployment,
            licenses,
            qualifications,
            hide_qualified: volunteer.hide_qualified.unwrap_or_default(),
        }
    }
}
