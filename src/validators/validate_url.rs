// Author    : Axel Vallon
// Date      : 24.04.2022
// Place     : HEIG-VD, Vaud, Switzerland
// Objective : Library that allow the semmentic verication of an URL and allow to whitelist top level domain.

use regex::Regex;

// Function that allow to verify a URL
// whitelist : slice that allow to specify authorised top domain name. If None is specified, no whitelist is used.
pub fn validate_url(url: &str, whitelist: Option<&[&str]>) -> bool {
    let regex_top_domain = Regex::new(r"^\.[a-zA-Z.]{1,}[a-zA-Z]$").unwrap();
    let regex_url =
        Regex::new(r"^([a-z0-9A-Z]*://)?[-.a-z0-9A-Z]{3,}\.[a-zA-Z.]{1,}[a-zA-Z]$").unwrap();
    if let Some(v) = whitelist {
        if v.iter().any(|top_domain_name| !regex_top_domain.is_match(top_domain_name)) // check if topdomain name in whitelist are all correct
        || v.iter().all(|top_domain_name| !url.ends_with(top_domain_name))
        // check if the url finish with one of the whitelisted topdomain
        {
            return false;
        }
    }
    regex_url.is_match(url)
}

// TODO : implement unit testing
#[cfg(test)]
mod tests {
    use crate::validate_url;

    #[test]
    fn valid_basic_url() {
        assert!(
            validate_url("http://axel.ch", None),
            "Basic url schould pass"
        );
        assert!(
            validate_url("axel.ch", None),
            "Basic url without protocol schould pass"
        );
    }

    #[test]
    fn valid_protocol_part() {
        assert!(
            validate_url("http2://test.ch", None),
            "Protocol part of URL schould accept numbers"
        );
        assert!(
            validate_url("HTTP://test.ch", None),
            "Protocol part of URL schould accept upper case letters"
        );
        assert!(
            validate_url("hTTP2://test.ch", None),
            "Protocol part of URL schould accept number with upper and lower case letters"
        );
    }

    #[test]
    fn invalid_protocol_part() {
        assert!(
            !validate_url("-http://test.ch", None),
            "Protocol part of URL contains only number and letters, no other special char"
        );
        assert!(
            !validate_url("ht-tp://test.ch", None),
            "Protocol part of URL contains only number and letters, no other special char"
        );
        assert!(
            !validate_url("http:/1/test.ch", None),
            "Protocol part of URL schould look like PROTOCOL://"
        );
        assert!(
            !validate_url("http:/test.ch", None),
            "Protocol part of URL schould look like PROTOCOL://"
        );
        assert!(
            !validate_url("http//test.ch", None),
            "Protocol part of URL schould look like PROTOCOL://"
        );
    }

    #[test]
    fn valid_subdomain_part() {
        assert!(
            validate_url("http://TEST.ch", None),
            "Subdomain part of URL schould accept upper case letters"
        );
        assert!(
            validate_url("http://test-test.ch", None),
            "Subdomain part of URL schould accept hyphens characters"
        );
        assert!(
            validate_url("http://www.test.ch", None),
            "Subdomain part of URL schould accept full stops characters"
        );
        assert!(
            validate_url("http://000.ch", None),
            "Subdomain part of URL schould accept numbers characters"
        );
        assert!(
            validate_url("http://2tes-t.test2-test.ch", None),
            "Subdomain part of URL schould accept the combinaison of accepted character"
        );
    }

    #[test]
    fn invalid_subdomain_part() {
        assert!(
            !validate_url("http://te.ch", None),
            "Subdomain part schould be at least 3 characters long"
        );
        assert!(
            !validate_url("http://t/s.ch", None),
            "Subdomain part accept only hyphens, number and letters"
        );
    }

    #[test]
    fn valid_top_domain_part() {
        //assert!(validate_url("http://test.2play", None), "Top domain part of URI schould accept numbers");
        assert!(
            validate_url("http://test.CH", None),
            "Top domain part of URI schould accept upper case letters"
        );
    }

    #[test]
    fn invalid_top_domain_part() {
        assert!(
            !validate_url("http://test.ch2", None),
            "Top domain schould finish with a letter"
        );
        assert!(
            !validate_url("http://text.c-h", None),
            "Top domain part of URI schould accept special characters"
        );
        assert!(
            !validate_url("http://text.c", None),
            "Top domain part of URI schould be 3 characters long"
        );
    }

    #[test]
    fn valid_whitelist() {
        assert!(
            validate_url("http://test.ch", Some(&[".ch", ".ww", ".org"])),
            "Basic valid whitelist"
        );
    }

    #[test]
    fn invalid_whitelist() {
        assert!(
            !validate_url("http://test.fr", Some(&[".ch", ".www", ".org"])),
            "URL topdomain not whitelisted"
        );
        assert!(
            !validate_url("http://test.fr", Some(&[".c"])),
            "Invalid topdomain schould not pass"
        );
        assert!(
            !validate_url("http://test.fr", Some(&[".c", ".ch", ".com"])),
            "One invalid topdomain in whitelist schould no pass"
        );
    }
}
