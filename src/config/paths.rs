use super::error::{ConfigError, Result};
use dirs::home_dir;
use std::path::{Path, PathBuf};

struct PathResolver {
    base_dir: PathBuf,
}

impl PathResolver {
    fn new(base_dir: &str) -> Result<Self> {
        let base_dir = if let Some(stripped) = base_dir.strip_prefix('~') {
            let home_dir = home_dir().ok_or(ConfigError::NoHomeDir)?;
            let relative_path = stripped.trim_start_matches('/');
            home_dir.join(relative_path)
        } else {
            PathBuf::from(base_dir)
        };

        if !base_dir.exists() {
            return Err(ConfigError::DirectoryNotFound(format!(
                "Base directory does not exist: {}",
                base_dir.display()
            )));
        }
        Ok(Self { base_dir })
    }

    fn resolve_input(&self, input_path: &str) -> Result<PathBuf> {
        let path = Path::new(input_path);
        let file_stem = path
            .file_stem()
            .ok_or_else(|| ConfigError::InvalidPath("Input path has no file name".to_string()))?;

        // Handle paths with directories
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let full_parent = self.base_dir.join(parent);

        // Try with provided extension if any
        if let Some(_ext) = path.extension() {
            let full_path = full_parent.join(path.file_name().unwrap());
            if full_path.exists() {
                return Ok(full_path);
            }
        }

        // Try .json
        let json_path = full_parent.join(file_stem).with_extension("json");
        if json_path.exists() {
            return Ok(json_path);
        }

        // Try .txt
        let txt_path = full_parent.join(file_stem).with_extension("txt");
        if txt_path.exists() {
            return Ok(txt_path);
        }

        Err(ConfigError::FileNotFound(format!(
            "Could not find input file: tried {:?} and {:?}",
            json_path, txt_path
        )))
    }

    fn resolve_output(
        &self,
        output_path: Option<&str>,
        input_path: &Path,
        inline_output: bool,
    ) -> Result<PathBuf> {
        match output_path {
            Some(path) => {
                if inline_output {
                    // Force the output path to be the same directory as the input path
                    let input_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
                    let output_file_name =
                        Path::new(path).file_name().unwrap_or_else(|| path.as_ref());

                    if let Some(_ext) = Path::new(path).extension() {
                        // Use provided extension
                        Ok(input_dir.join(output_file_name))
                    } else {
                        // Use .md extension
                        Ok(input_dir.join(output_file_name).with_extension("md"))
                    }
                } else {
                    let path = Path::new(path);
                    let parent = path.parent().unwrap_or_else(|| Path::new(""));
                    let full_parent = self.base_dir.join(parent);

                    if let Some(_ext) = path.extension() {
                        // Use provided extension
                        Ok(full_parent.join(path.file_name().unwrap()))
                    } else {
                        // Use .md extension
                        Ok(full_parent
                            .join(path.file_stem().unwrap())
                            .with_extension("md"))
                    }
                }
            }
            None => {
                if inline_output {
                    // Force output to be in the same directory as the input file
                    let input_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
                    let stem = input_path.file_stem().ok_or_else(|| {
                        ConfigError::InvalidPath("Input path has no file stem".to_string())
                    })?;
                    Ok(input_dir.join(stem).with_extension("md"))
                } else {
                    // Get the relative part of the input path from base_dir
                    let relative_input = input_path
                        .strip_prefix(&self.base_dir)
                        .unwrap_or_else(|_| Path::new(input_path.file_name().unwrap_or_default()));

                    // Get the parent directory relative to base_dir
                    let parent = relative_input.parent().unwrap_or_else(|| Path::new(""));
                    let stem = relative_input.file_stem().ok_or_else(|| {
                        ConfigError::InvalidPath("Input path has no file stem".to_string())
                    })?;

                    // Join with base_dir to create output path
                    Ok(self.base_dir.join(parent).join(stem).with_extension("md"))
                }
            }
        }
    }
}

pub fn resolve_paths(
    base_dir: &str,
    inline_output: bool,
    input_file: &str,
    output_file: Option<&str>,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let resolver = PathResolver::new(base_dir)?;
    let input_path = resolver.resolve_input(input_file)?;
    let output_path = resolver.resolve_output(output_file, &input_path, inline_output)?;

    Ok((resolver.base_dir, input_path, output_path))
}
