use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ImageMagick {
    pub path: String,
}

impl ImageMagick {
    pub fn convert(&self, source_file: &Path, target_file: &Path) -> String {
        let process = std::process::Command::new(self.path.as_str())
            .arg(source_file)
            .arg("-nologo")
            .arg("-ft")
            .arg("png")
            .arg("-y")
            .arg("-o")
            .arg(target_file.parent().unwrap().to_str().unwrap())
            .arg("-srgb")
            .arg("-f")
            .arg("R8G8B8A8_UNORM_SRGB")
            .output();
        if let Ok(output) = process {
            if !output.status.success() {
                println!("{:?}", String::from_utf8_lossy(&output.stdout));
                return String::from_utf8_lossy(&output.stdout).to_string();
            }
            return String::from("")
        }
        return String::from("error when converting")
        // let process = std::process::Command::new(self.path.as_str())
        //     .arg("convert")
        //     .arg(source_file)
        //     .arg("-auto-orient")
        //     .arg("-alpha")
        //     .arg("off")
        //     .arg(target_file)
        //     .output();
        // if let Ok(output) = process {
        //     if !output.status.success() {
        //         println!("{:?}", String::from_utf8_lossy(&output.stderr));
        //         let process = std::process::Command::new(self.path.replace("magick.exe", "texconv.exe").as_str())
        //             .arg(source_file)
        //             .arg("-ft")
        //             .arg("png")
        //             .arg("-y")
        //             .arg("-o")
        //             .arg(target_file.parent().unwrap().to_str().unwrap())
        //             .arg("-wiclossless")
        //             .arg("-nologo")
        //             .output();
        //         if let Ok(output) = process {
        //             if !output.status.success() {
        //                 println!("{:?}", String::from_utf8_lossy(&output.stderr));
        //                 return String::from_utf8_lossy(&output.stderr).to_string();
        //             }
        //             return String::from("")
        //         }
        //         return String::from_utf8_lossy(&output.stderr).to_string();
        //     }
        //     return String::from("")
        // }
    }

    pub fn convert_to_png(&self, source_file: &Path) -> Option<PathBuf> {
        if let Some(_) = source_file.file_name() {
            let mut outname = String::from(source_file.file_stem().unwrap().to_str().unwrap());
            outname.push_str(".png");
            let basepath = source_file.parent().unwrap();
            let outpath = basepath.join(outname);
            let result = self.convert(source_file, outpath.as_path());
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
        let magick = ImageMagick::default();
        let source = Path::new(r"./anbennar/gfx/flags/A03.tga");
        assert!(!magick.convert_to_png(source).is_none());
    }
}
