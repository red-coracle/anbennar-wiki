use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ImageMagick {
    pub path: String,
}

impl ImageMagick {
    pub fn convert(&self, source_file: &Path, target_file: &Path) -> String {
        // println!("{:?}", source_file);
        // println!("{:?}", target_file);
        // println!("{}", self.path.as_str());
        let process = std::process::Command::new(self.path.as_str())
            .arg(source_file)
            .arg(target_file)
            .output();
        // println!("{:?}", process);
        if let Ok(output) = process {
            if !output.status.success() {
                println!("{:?}", String::from_utf8_lossy(&output.stdout));
                return String::from_utf8_lossy(&output.stdout).to_string();
            }
            return String::from("")
        }
        return String::from("error when converting")
    }

    pub fn convert_to_png(&self, source_file: &Path) -> Option<PathBuf> {
        if let Some(_) = source_file.file_name() {
            let mut outname = String::from(source_file.file_stem().unwrap().to_str().unwrap());
            outname.push_str(".png");
            let basepath = source_file.parent().unwrap();
            let outpath = basepath.join(outname);
            let result = self.convert(source_file, outpath.as_path());
            // println!("ERG {}", result);
            if result == "" {
                return Some(outpath)
            }
        }
        return None
    }
}

impl Default for ImageMagick {
    fn default() -> Self {
        ImageMagick{
            path: "./magick/texconv.exe".to_string()
        }
    }
}

impl From<&str> for ImageMagick {
    fn from(path: &str) -> Self {
        ImageMagick{
            path: path.to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_convert() {
    }

    #[test]
    pub fn test_convert_to_png() {
        let magick = ImageMagick::from("convert");
        let source = Path::new(r"./anbennar/gfx/flags/A03.tga");
        assert!(!magick.convert_to_png(source).is_none());
    }
}
