use std::io::Error;
use infer::Type;

// Function that allow to verify if the path given in "path" is a valid image or a video.
// path : the relative path to the file we want to check.
// verify_extension : if true, we will check the extension in the file name is the same as the content of the file. 
// Warning: The extension in filename must have 3 char (ex: jpg and not jpeg).
// Return the following content : 
// -> IO:Error if the file doesn't exist or is not readable
// -> Ok(None) if the file is not a video or an image
// -> Ok(Some(&str)) If the file is an image, with the fixed content "image" or "video" associated
pub fn validate_file(path: &String, verify_extension: bool)->Result<Option<&str>, Error> {
    let kind = infer::get_from_path(path)?;
    match kind { 
        Some(file_type) => {
            // if we don't want to check the extension, we check directely if it's an image or a video
            if !verify_extension || match_extension(path, &file_type) {
                match file_type.matcher_type() {
                    infer::MatcherType::Image => Ok(Some("image")),
                    infer::MatcherType::Video => Ok(Some("video")),
                    _  => Ok(None) // not an image nor a video
                }
            } else {
                Ok(None) // extension in filename don't match exension in header
            }
        },
        None => Ok(None) // the extension is not known. thus it is not accepted as an image or a video.
    }
}

// the comparaison is true if the filename in extension is the same as the content of file and has 3 characters lenght (jpg and not jpeg or tif and not tiff)
fn match_extension(path: &String, type_file: &Type) -> bool {
    path.to_lowercase().ends_with(type_file.extension())
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {

    use std::fs;
    use crate::validate_file;
    const IMAGES_PATH: &str = "res/image";
    const VIDEO_PATH: &str = "res/video";
    const OTHER_PATH: &str = "res/other_extension";
    const MODIFIED_EXTENSION: &str = "res/image_with_modified_extension"; // .csv file don't have header


    #[test]
    fn corrects_image() {
        let paths = fs::read_dir(IMAGES_PATH).unwrap();
        for image_path in paths {
            assert_eq!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), true).unwrap(), Some("image"));
        }
    }

    #[test]
    fn corrects_image_without_ext_verifcation() {
        let paths = fs::read_dir(IMAGES_PATH).unwrap();
        for image_path in paths {
            assert_eq!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), false).unwrap(), Some("image"));
        }
    }



    #[test]
    fn corrects_video() {
        let paths = fs::read_dir(VIDEO_PATH).unwrap();
        for image_path in paths {
            assert_eq!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), true).unwrap(), Some("video"));
        }
    }

    #[test]
    fn incorrect_other_file() {
        let paths = fs::read_dir(OTHER_PATH).unwrap();
        for image_path in paths {
            assert!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), true).unwrap().is_none());
        }
    }

    #[test]
    fn incorrect_extension_in_filename() {
        let paths = fs::read_dir(MODIFIED_EXTENSION).unwrap();
        for image_path in paths {
            assert!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), true).unwrap().is_none());
        }
    }

    #[test]
    fn incorrect_extension_with_no_verification_of_extension() {
        let paths = fs::read_dir(MODIFIED_EXTENSION).unwrap();
        for image_path in paths {
            assert_eq!(validate_file(&image_path.unwrap().path().to_str().unwrap().to_string(), false).unwrap(), Some("image"));
        }
    }


    #[test]
    fn inexistant_file() {
        assert!(validate_file(&"file_that_schould_not_exist.txt".to_string(), true).is_err());
    }
}
