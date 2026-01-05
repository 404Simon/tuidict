use super::api::FreeDictEntry;
use anyhow::{anyhow, Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tar::Archive;

/// downloads file with progress tracking
pub fn download_file<F>(
    url: &str,
    output_path: &Path,
    total_size: u64,
    progress_callback: F,
) -> Result<()>
where
    F: Fn(u64, u64),
{
    let mut response = reqwest::blocking::get(url).context("Failed to download file")?;

    let mut file = File::create(output_path).context("Failed to create output file")?;

    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = response
            .read(&mut buffer)
            .context("Failed to read from response")?;

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])
            .context("Failed to write to file")?;

        downloaded += bytes_read as u64;
        progress_callback(downloaded, total_size);
    }

    Ok(())
}

pub fn extract_tar_xz(tar_path: &Path, output_dir: &Path) -> Result<()> {
    let tar_file = File::open(tar_path).context("Failed to open tar file")?;

    let xz_decoder = xz2::read::XzDecoder::new(tar_file);
    let mut archive = Archive::new(xz_decoder);
    archive
        .unpack(output_dir)
        .context("Failed to extract tar archive")?;

    Ok(())
}

pub fn find_dict_files(dict_dir: &Path) -> Result<(PathBuf, PathBuf)> {
    let mut index_path = None;
    let mut dict_path = None;

    for entry in fs::read_dir(dict_dir).context("Failed to read dictionary directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if filename.ends_with(".index") {
                index_path = Some(path.clone());
            } else if filename.ends_with(".dict.dz") {
                dict_path = Some(path.clone());
            }
        }
    }

    // check subdirectories
    if index_path.is_none() || dict_path.is_none() {
        for entry in fs::read_dir(dict_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                for subentry in fs::read_dir(&path)? {
                    let subentry = subentry?;
                    let subpath = subentry.path();

                    if subpath.is_file() {
                        let filename = subpath.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        if filename.ends_with(".index") && index_path.is_none() {
                            index_path = Some(subpath.clone());
                        } else if filename.ends_with(".dict.dz") && dict_path.is_none() {
                            dict_path = Some(subpath.clone());
                        }
                    }
                }
            }
        }
    }

    match (index_path, dict_path) {
        (Some(index), Some(dict)) => Ok((index, dict)),
        _ => Err(anyhow!("Could not find .index or .dict.dz files")),
    }
}

pub fn download_and_install<F>(
    entry: &FreeDictEntry,
    target_dir: &Path,
    progress_callback: F,
) -> Result<PathBuf>
where
    F: Fn(u64, u64) + Send + 'static,
{
    let release = entry
        .get_dictd_release()
        .ok_or_else(|| anyhow!("No dictd release available"))?;

    let temp_dir = target_dir.join(".tmp");
    fs::create_dir_all(&temp_dir).context("Failed to create temp directory")?;

    let dict_id = entry.name.clone();

    let tar_path = temp_dir.join(format!("{}.tar.xz", dict_id));
    download_file(&release.url, &tar_path, release.size, progress_callback)?;

    let dict_dir = target_dir.join(&dict_id);
    fs::create_dir_all(&dict_dir).context("Failed to create dictionary directory")?;

    extract_tar_xz(&tar_path, &dict_dir)?;

    let _ = fs::remove_file(&tar_path);
    let _ = fs::remove_dir(&temp_dir);

    Ok(dict_dir)
}
