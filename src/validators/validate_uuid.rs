// Author    : Axel Vallon
// Date      : 24.04.2022
// Place     : HEIG-VD, Vaud, Switzerland
// Objective : Library that allow the semmentic verication of UUID and verify if the content of a file match an UUID.

use regex::Regex;
use std::io::Error;
use uuid::Uuid;

// Allow to validate the format of a v5 UUID in hyphenated format
pub fn validate_uuid(uuid: &str) -> bool {
    let regex_uuid =
        Regex::new(r"^[[:xdigit:]]{8}-([[:xdigit:]]{4}-){3}[[:xdigit:]]{12}$").unwrap();
    regex_uuid.is_match(uuid)
}

// Allow to validate that a UUID in correctly linked to a file content.
// No unitary test is done but this function as been verified manually.
pub fn validate_file_with_uuid(uuid: &str, filename: &str) -> Result<bool, Error> {
    let buffer = std::fs::read(filename)?;
    Ok(Uuid::new_v5(&Uuid::default(), &buffer)
        .as_hyphenated()
        .to_string()
        == uuid)
}

#[cfg(test)]
mod tests {
    use crate::validate_uuid;

    #[test]
    fn valid_uuid() {
        assert!(
            validate_uuid("00000000-0000-0000-0000-000000000000"),
            "UUID with full 0 is ok"
        );
        assert!(
            validate_uuid("936DA01F-9ABD-4D9D-80C7-02AF85C822A8"),
            "UUID random with uppercase schould be ok"
        );
        assert!(
            validate_uuid("936da01f-9abd-4d9d-80c7-02af85c822a8"),
            "UUID random with lowercase schould be ok"
        );
        assert!(
            validate_uuid("936dA01f-9abd-4d9d-80C7-02Af85c822A8"),
            "UUID random with lowercase and uppercase schould be ok"
        );
    }

    #[test]
    fn invalid_uuid_struct() {
        assert!(
            !validate_uuid("0000000-0000-0000-0000-000000000000"),
            "Invalid number of el in first block"
        );
        assert!(
            !validate_uuid("00000000000000000000000000000000"),
            "Miss hyphen char"
        );
        assert!(
            !validate_uuid("00000000-0000-0000-0000000000000000"),
            "Miss one block in the middle"
        );
        assert!(
            !validate_uuid("00000000-0000-0000-0000-00000000000"),
            "Miss one char at then end"
        );
    }

    #[test]
    fn invalid_char_in_uuid() {
        assert!(
            !validate_uuid("zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz"),
            "Only hex char are valid in UUID"
        );
        assert!(
            !validate_uuid("z.1zzz/z-zzzz-zzzz-zzzz-zzzz''zzzzzz"),
            "Only hex char are valid in UUID"
        );
    }
}
