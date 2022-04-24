// Author    : Axel Vallon
// Date      : 24.04.2022
// Place     : HEIG-VD, Vaud, Switzerland
// Objective : A library that allow the verification of a file at a given path. It must be an image or a video. Allow to verify the filename extension too.

use infer::Type;
use std::io::Error;

// Function that allow to verify if the path given in "path" is a valid image or a video.
// path : the relative path to the file we want to check.
// verify_extension : if true, we will check the extension in the file name is the same as the content of the file.

// Return the following content :
// -> IO:Error if the file doesn't exist or is not readable
// -> Ok(None) if the file is not a video or an image
// -> Ok(Some(&str)) If the file is an image, with the fixed content "image" or "video" associated
pub fn validate_file(path: &String, verify_extension: bool) -> Result<Option<&str>, Error> {
    let kind = infer::get_from_path(path)?;
    match kind {
        Some(file_type) => {
            // if we don't want to check the extension, we check directely if it's an image or a video
            if !verify_extension || match_extension(path, &file_type) {
                match file_type.matcher_type() {
                    infer::MatcherType::Image => Ok(Some("image")),
                    infer::MatcherType::Video => Ok(Some("video")),
                    _ => Ok(None), // not an image nor a video
                }
            } else {
                Ok(None) // extension in filename don't match exension in header
            }
        }
        None => Ok(None), // the extension is not known. thus it is not accepted as an image or a video.
    }
}

// the comparaison is true if the filename in extension is the same as the content of file
fn match_extension(path: &String, type_file: &Type) -> bool {
    if path.ends_with(".jpeg") {
        return type_file.extension() == "jpg";
    } else if path.ends_with(".tiff") {
        return type_file.extension() == "tif";
    }
    path.to_lowercase().ends_with(type_file.extension())
}

#[cfg(test)]
mod tests {

    use crate::validate_file;
    use std::fs;
    const IMAGES_PATH: &str = "res/image";
    const VIDEO_PATH: &str = "res/video";
    const OTHER_PATH: &str = "res/other_extension";
    const MODIFIED_EXTENSION: &str = "res/image_with_modified_extension";

    #[test]
    fn corrects_image() {
        let paths = fs::read_dir(IMAGES_PATH).unwrap();
        for image_path in paths {
            assert_eq!(
                validate_file(
                    &image_path.unwrap().path().to_str().unwrap().to_string(),
                    true
                )
                .unwrap(),
                Some("image"),
                "image file schould pass"
            );
        }
    }

    #[test]
    fn corrects_image_without_ext_verifcation() {
        let paths = fs::read_dir(IMAGES_PATH).unwrap();
        for image_path in paths {
            assert_eq!(
                validate_file(
                    &image_path.unwrap().path().to_str().unwrap().to_string(),
                    false
                )
                .unwrap(),
                Some("image"),
                "image file without extension verification schould pass"
            );
        }
    }

    #[test]
    fn corrects_video() {
        let paths = fs::read_dir(VIDEO_PATH).unwrap();
        for image_path in paths {
            assert_eq!(
                validate_file(
                    &image_path.unwrap().path().to_str().unwrap().to_string(),
                    true
                )
                .unwrap(),
                Some("video"),
                "video file schould be ok"
            );
        }
    }

    #[test]
    fn incorrect_other_file() {
        let paths = fs::read_dir(OTHER_PATH).unwrap();
        for image_path in paths {
            assert!(
                validate_file(
                    &image_path.unwrap().path().to_str().unwrap().to_string(),
                    true
                )
                .unwrap()
                .is_none(),
                "file that is not a video nor a image schould not pass"
            );
        }
    }

    #[test]
    fn incorrect_extension_in_filename() {
        let paths = fs::read_dir(MODIFIED_EXTENSION).unwrap();
        for image_path in paths {
            assert!(validate_file(
                &image_path.unwrap().path().to_str().unwrap().to_string(),
                true
            )
            .unwrap()
            .is_none(),
            "An image with modified extension in filename schould not pass if the extension validation is asked");
        }
    }

    #[test]
    fn incorrect_extension_with_no_verification_of_extension() {
        let paths = fs::read_dir(MODIFIED_EXTENSION).unwrap();
        for image_path in paths {
            assert_eq!(
                validate_file(
                    &image_path.unwrap().path().to_str().unwrap().to_string(),
                    false
                )
                .unwrap(),
                Some("image"),
                "An image with modified extension in filename schould pass if the extension validation in not asked"
            );
        }
    }

    #[test]
    fn inexistant_file() {
        assert!(
            validate_file(&"file_that_schould_not_exist.txt".to_string(), true).is_err(),
            "An inexistant file schould generate an Error"
        );
    }
}
