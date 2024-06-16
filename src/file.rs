use directories::ProjectDirs;
use flate2::read::GzDecoder;
use rmp_serde;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use tar::Archive;

use crate::access::data::ParsecData;
use crate::access::masses::{get_filenames, get_masses};
use crate::access::metallicity::Metallicity;
use crate::access::PARSEC_URL;
use crate::error::ParsecAccessError;
use crate::line::ParsedParsecLine;
use crate::trajectory::Trajectory;

impl ParsecData {
    pub(crate) fn new(metallicity: Metallicity) -> Result<ParsecData, ParsecAccessError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let file_path = data_dir.join(metallicity.to_string() + ".rmp");

        if file_path.exists() {
            println!("Reading PARSEC data from {}", file_path.display());
            let file = File::open(&file_path).map_err(ParsecAccessError::Io)?;
            let parsec_data: ParsecData =
                rmp_serde::from_read(file).map_err(ParsecAccessError::RmpDeserialization)?;
            if parsec_data.is_filled() {
                Ok(parsec_data)
            } else {
                Err(ParsecAccessError::DataNotAvailable(
                    "Parsec Data".to_string(),
                ))
            }
        } else {
            Self::ensure_data_files(&metallicity)?;
            let data_dir_name = metallicity.to_archive_name().replace(".tar.gz", "");
            let folder_path = data_dir.join(PathBuf::from(data_dir_name));
            let masses = get_masses(&metallicity);
            let filepaths = get_filenames(&metallicity);
            let mut parsec_data = ParsecData {
                metallicity,
                data: Vec::with_capacity(masses.len()),
            };
            for mass_index in 0..masses.len() {
                let filepath = folder_path.join(filepaths[mass_index]);
                parsec_data.data.push(Self::read_file(filepath)?);
            }
            println!("Writing PARSEC data to {}", file_path.display());
            let file = File::create(&file_path).map_err(ParsecAccessError::Io)?;
            let buffer =
                rmp_serde::to_vec(&parsec_data).map_err(ParsecAccessError::RmpSerialization)?;
            let mut writer = BufWriter::new(file);
            writer.write_all(&buffer).map_err(ParsecAccessError::Io)?;
            Self::delete_data_files(&metallicity)?;
            if parsec_data.is_filled() {
                Ok(parsec_data)
            } else {
                Err(ParsecAccessError::DataNotAvailable(
                    "Parsec Data".to_string(),
                ))
            }
        }
    }

    fn download(metallicity: &Metallicity) -> Result<(), ParsecAccessError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let data_dir = data_dir
            .to_str()
            .ok_or(ParsecAccessError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert data dir to string",
            )))?;
        println!(
            "Downloading PARSEC data for {} to {}",
            metallicity, data_dir
        );
        let target = PARSEC_URL.to_string() + metallicity.to_archive_name();
        let mut response = reqwest::blocking::get(target).map_err(ParsecAccessError::Connection)?;
        let gz_decoder = GzDecoder::new(&mut response);
        let mut archive = Archive::new(gz_decoder);
        archive.unpack(data_dir).map_err(ParsecAccessError::Io)?;
        Ok(())
    }

    fn ensure_data_files(metallicity: &Metallicity) -> Result<(), ParsecAccessError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let dirname = metallicity.to_archive_name().replace(".tar.gz", "");
        let path = data_dir.join(PathBuf::from(dirname));
        if !path.exists() {
            Self::download(metallicity)?;
        }
        Ok(())
    }

    fn delete_data_files(metallicity: &Metallicity) -> Result<(), ParsecAccessError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let dirname = metallicity.to_archive_name().replace(".tar.gz", "");
        let path = data_dir.join(PathBuf::from(dirname));
        if path.exists() {
            fs::remove_dir_all(path).map_err(ParsecAccessError::Io)?;
        }
        Ok(())
    }

    fn read_file(file_path: PathBuf) -> Result<Trajectory, ParsecAccessError> {
        let file = File::open(file_path).map_err(ParsecAccessError::Io)?;
        let reader = BufReader::new(file);
        let mut lines = vec![];
        for line in reader.lines() {
            let line = line.map_err(ParsecAccessError::Io)?;
            if !is_header(&line) {
                let line = ParsedParsecLine::read(line)?;
                lines.push(line);
            }
        }
        Ok(Trajectory::new(lines))
    }
}

fn is_header(line: &String) -> bool {
    line.chars()
        .any(|c| c.is_alphabetic() && c != 'E' && c != 'e')
}

fn get_project_dirs() -> Result<ProjectDirs, ParsecAccessError> {
    // TODO: Include version in project dirs
    ProjectDirs::from("", "the_comamba", "parsec_access").ok_or(ParsecAccessError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, "Could not get project dirs"),
    ))
}
