use directories::ProjectDirs;
use flate2::read::GzDecoder;
use glob::glob;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tar::Archive;

use crate::access::masses::FILENAMES;
use crate::access::metallicity::{
    METALLICITIES_IN_MASS_FRACTION, METALLICITY_ARCHIVES, METALLICITY_NAMES,
};
use crate::access::PARSEC_URL;
use crate::data::ParsecData;
use crate::error::ParsecAccessError;
use crate::line::ParsecLine;
use crate::trajectory::Trajectory;
use crate::{PACKAGE_NAME, PACKAGE_VERSION};

impl ParsecData {}

fn download(metallicity_index: usize) -> Result<(), ParsecAccessError> {
    let data_dir = get_data_dir()?;
    let data_dir = data_dir
        .to_str()
        .ok_or(ParsecAccessError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert data dir to string",
        )))?;
    let archive_name = METALLICITY_ARCHIVES[metallicity_index];
    println!(
        "Downloading PARSEC data archive {} to {}",
        archive_name, data_dir
    );
    let target = PARSEC_URL.to_string() + archive_name;
    let mut response = reqwest::blocking::get(target).map_err(ParsecAccessError::Connection)?;
    let gz_decoder = GzDecoder::new(&mut response);
    let mut archive = Archive::new(gz_decoder);
    archive.unpack(data_dir).map_err(ParsecAccessError::Io)?;
    Ok(())
}

fn read_trajectory_file(file_path: PathBuf) -> Result<Trajectory, ParsecAccessError> {
    let file = File::open(file_path).map_err(ParsecAccessError::Io)?;
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.map_err(ParsecAccessError::Io)?;
        if !is_header(&line) {
            let line = ParsecLine::read(line)?;
            lines.push(line);
        }
    }
    Ok(Trajectory::new(lines))
}

fn ensure_data_files(metallicity_index: usize) -> Result<(), ParsecAccessError> {
    let data_dir = get_data_dir()?;
    let dirname = METALLICITY_ARCHIVES[metallicity_index].replace(".tar.gz", "");
    let path = data_dir.join(PathBuf::from(dirname));
    if !path.exists() {
        download(metallicity_index)?;
        reduce_persisted_data(metallicity_index)?;
    }
    clean_up_old_data_dirs()?;
    Ok(())
}

fn clean_up_old_data_dirs() -> Result<(), ParsecAccessError> {
    let data_dir = get_data_dir()?;
    let data_dir_str = data_dir
        .to_str()
        .ok_or(ParsecAccessError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert data dir to string",
        )))?;
    let parts: Vec<&str> = data_dir_str.split('_').collect();
    let data_dir_glob = parts[..parts.len() - 1].join("_") + "_*";

    let entries = glob(&data_dir_glob).map_err(ParsecAccessError::GlobPattern)?;
    for entry in entries {
        let path = entry.map_err(ParsecAccessError::Glob)?;
        if path != data_dir {
            println!("Removing old data directory: {:?}", path);
            fs::remove_dir_all(&path).map_err(ParsecAccessError::Io)?;
        }
    }
    Ok(())
}

fn reduce_persisted_data(metallicity_index: usize) -> Result<(), ParsecAccessError> {
    let data_dir = get_data_dir()?;
    let data_dir_name = METALLICITY_ARCHIVES[metallicity_index].replace(".tar.gz", "");
    let folder_path = data_dir.join(PathBuf::from(data_dir_name));
    delete_unnecessary_files(&folder_path)?;
    trim_files(&folder_path, metallicity_index)?;
    Ok(())
}

fn delete_unnecessary_files(folder_path: &PathBuf) -> Result<(), ParsecAccessError> {
    println!(
        "Deleting unnecessary files in {}",
        folder_path.to_string_lossy()
    );

    let pattern = format!("{}/**/*HB.DAT", folder_path.to_string_lossy());

    for entry in glob(&pattern).map_err(ParsecAccessError::GlobPattern)? {
        let entry = entry.map_err(ParsecAccessError::Glob)?;
        fs::remove_file(entry).map_err(ParsecAccessError::Io)?;
    }

    let pattern = format!("{}/**/*ADD.DAT", folder_path.to_string_lossy());

    for entry in glob(&pattern).map_err(ParsecAccessError::GlobPattern)? {
        let entry = entry.map_err(ParsecAccessError::Glob)?;
        fs::remove_file(entry).map_err(ParsecAccessError::Io)?;
    }

    Ok(())
}

fn trim_files(folder_path: &PathBuf, metallicity_index: usize) -> Result<(), ParsecAccessError> {
    println!("Trimming files in {}", folder_path.to_string_lossy());

    let required_line_number = ParsecLine::LARGEST_REQUIRED_INDEX + 1;
    let filepaths = FILENAMES[metallicity_index];
    for filepath in filepaths {
        let filepath = folder_path.join(filepath);
        trim_file(&filepath, required_line_number)?;
    }

    Ok(())
}

fn trim_file(file_path: &PathBuf, required_line_number: usize) -> Result<(), ParsecAccessError> {
    let file = File::open(file_path).map_err(ParsecAccessError::Io)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(ParsecAccessError::Io)?;
        let columns: Vec<&str> = line.split_whitespace().collect();
        let trimmed_columns = columns
            .into_iter()
            .take(required_line_number)
            .collect::<Vec<&str>>()
            .join("\t");
        lines.push(trimmed_columns);
    }

    fs::write(file_path, lines.join("\n")).map_err(ParsecAccessError::Io)?;
    Ok(())
}

pub(crate) fn read_data_files(
    metallicity_index: usize,
    data_dir: &PathBuf,
) -> Result<ParsecData, ParsecAccessError> {
    let parsec_data = read_parsec_data_from_files(metallicity_index, data_dir)?;

    if parsec_data.is_valid() {
        Ok(parsec_data)
    } else {
        let metallicity = METALLICITY_NAMES[metallicity_index];
        Err(ParsecAccessError::DataNotAvailable(format!(
            "Parsec Data for metallicity {} is empty.",
            metallicity
        )))
    }
}

fn read_parsec_data_from_files(
    metallicity_index: usize,
    data_dir: &PathBuf,
) -> Result<ParsecData, ParsecAccessError> {
    ensure_data_files(metallicity_index)?;
    let data_dir_name = METALLICITY_ARCHIVES[metallicity_index].replace(".tar.gz", "");
    let folder_path = data_dir.join(PathBuf::from(data_dir_name));
    let filepaths = FILENAMES[metallicity_index];
    let mut parsec_data = ParsecData {
        metallicity_in_mass_fraction: METALLICITIES_IN_MASS_FRACTION[metallicity_index],
        data: Vec::new(),
    };

    let data: Vec<_> = filepaths
        .par_iter()
        .map(|filepath| {
            let filepath = folder_path.join(filepath);
            read_trajectory_file(filepath)
        })
        .collect::<Result<_, _>>()?;

    parsec_data.data.extend(data);
    Ok(parsec_data)
}

fn is_header(line: &String) -> bool {
    line.chars()
        .any(|c| c.is_alphabetic() && c != 'E' && c != 'e')
}

pub(crate) fn get_data_dir() -> Result<PathBuf, ParsecAccessError> {
    let error = ParsecAccessError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not get project dirs",
    ));
    let app = format!("{}_{}", PACKAGE_NAME, PACKAGE_VERSION);
    let project_dirs = ProjectDirs::from("", "the_comamba", &app).ok_or(error)?;
    Ok(project_dirs.data_dir().into())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore] // This test manipulates the data files while other tests try to read them
    fn reducing_data() {
        for (metallicity_index, _) in METALLICITIES_IN_MASS_FRACTION.iter().enumerate() {
            let result = ensure_data_files(metallicity_index);
            assert!(result.is_ok(), "{}", result.unwrap_err());
            let result = reduce_persisted_data(metallicity_index);
            assert!(result.is_ok(), "{}", result.unwrap_err());
        }
    }
}
