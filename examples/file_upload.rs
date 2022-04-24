// Author    : Axel Vallon
// Date      : 24.04.2022
// Place     : HEIG-VD, Vaud, Switzerland
// Objective : An example main that use the validators library to upload, verify and get the path of a video or image file.

use lab01_2022_input_validation::*;
use lazy_static::lazy_static;
use read_input::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use uuid::Uuid;

// store the content we need from a file.
#[derive(Clone)]
struct MediaFile {
    path: String,
    media_type: String,
}

// Hashmap that store the saved relation UUID -> MediaFile
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, MediaFile>> = Mutex::new(HashMap::new()); // str faster for UUID but ref problems occured in the implementation
}

// Function that upload a MediaFile with an associated UUID.
// Return : True if the upload is successuful. False if the file already exist, and the content is not uploaded/modified.
fn upload_file(uuid: &String, filepath: &String, media_type: String) -> bool {
    let mut map = HASHMAP.lock().unwrap();
    match map.get(uuid) {
        Some(_) => false, // already is in the Hashmap
        None => {
            map.insert(
                uuid.clone(),
                MediaFile {
                    path: filepath.clone(),
                    media_type: media_type.clone(),
                },
            );
            true
        }
    }
}

// Function that allow a user to upload a valid image or video with his file Path andrint differents output with differents errors cases.
// WARNING : Please modify these output for your own use case. This solution can leak some info. This is an example file upload case.
// The UUID is based on the file content. It include that a file can not be uploaded twice, even with 2 different names.
fn file_upload_handler() {
    loop {
        let filepath = input::<String>()
            .msg("Please enter the path to an image or video file : ")
            .get();
        match validate_file(&filepath, true) {
            Ok(media_type_opt) => {
                //The selected File exist
                match media_type_opt {
                    Some(media_type) => {
                        // The selected file is valid and we retrieve his media type ("video" or "image")
                        // we get the buffer to calculate the uuid with the file content with the default UUID v5 namespace (deterministic)
                        let buffer = &fs::read(&filepath).unwrap();
                        let uuid = Uuid::new_v5(&Uuid::default(), buffer)
                            .as_hyphenated()
                            .to_string();
                        if upload_file(&uuid, &filepath, media_type.to_string()) {
                            println!("File uploaded successfully, UUID : {}", uuid);
                            break; // correct input, we leave the loop
                        } else {
                            println!("This file already exists")
                        }
                    }
                    None => println!("{}", "Invalid file content"),
                }
            }
            Err(_) => println!("{}", "The file is not readable or doesn't exist"),
        }
    }
}

// Allow to retrieve a copy of the MediaFile in the Hashmap with the uuid key. Returh None if non-existant
fn retrieve_with_uuid(uuid: &String) -> Option<MediaFile> {
    let map = HASHMAP.lock().unwrap();
    match map.get(uuid) {
        Some(media_file) => Some(media_file.clone()),
        None => None,
    }
}

// Function that allow the verification of the existance of the file in the Hashmap
// This functilow a user to test many things :
// - Test of selected UUID format
// - Test the exitance in Hashmap of the file
// - Verification if the file still exist in the fs and if the content is the same.
fn file_verify_handler() {
    loop {
        let uuid_input = input::<String>()
            .msg("Please enter the UUID to check : ")
            .get();
        if !validate_uuid(&uuid_input) {
            println!("The provided UUID is not valid")
        } else {
            match retrieve_with_uuid(&uuid_input) {
                // we verify if it's saved
                Some(media_file) => {
                    match validate_file_with_uuid(&uuid_input, &media_file.path) {
                        // we verify if the content has been modified
                        Ok(identical_file) => {
                            if identical_file {
                                println!(
                                    "File {} exists, it is a/an {} file",
                                    uuid_input, media_file.media_type
                                );
                                break;
                            } else {
                                // Remove this if this leak info on your infra
                                println!("The file has been modified, the content is not the same");
                            }
                        } // Remove this if this leak info on your infra
                        Err(_) => println!("The file has been moved or don't exist anymore"),
                    }
                }
                None => println!("The file {} doesn't exist", uuid_input),
            }
        }
    }
}

// Function that return the URL of a file if it had been stored. No further verification is done on the file. Refer to file_verify_handler()
fn get_url_handler() {
    let uuid_input = input::<String>()
        .msg("Please enter the UUID to get : ")
        .get();
    if let Some(media_file) = retrieve_with_uuid(&uuid_input) {
        println!("sec.upload/{}s/{}", media_file.media_type, media_file.path)
    } else {
        // we only give the File not found info to user there. More info could leak something.
        println!("File not found")
    }
}

fn main() {
    println!("Welcome to the super secure file upload tool !");
    loop {
        match input::<i32>().repeat_msg("Please select one of the following options to continue :\n1 - Upload a file\n2 - Verify file exists\n3 - Get file URL\n0 - Exit\nYour input ? [0-3]")
            .min_max(0, 3).get() {
            0 => {
                println!("Goodbye!");
                break
            },
            1 => file_upload_handler(),
            2 => file_verify_handler(),
            3 => get_url_handler(),
            _ => panic!("Invalid input"),
        }
        println!();
    }
}
