use crate::container::Container;
use crate::parser::parse_str;

mod tests {
    use super::*;

    #[test]
    fn test_true_false() -> Result<(), Box<dyn core::error::Error>> {
        // Test wrong true false
        let p = parse_str(r#"[truer]"#);

        assert!(p.is_err());
        assert!(parse_str(r#"[falsei]"#).is_err());

        Ok(())
    }

    #[test]
    fn test_primitive() -> Result<(), Box<dyn core::error::Error>> {
        assert!(parse_str("12")
            .is_ok_and(|c| c.get_uint().is_some_and(|d| d == 12)));
        assert!(parse_str("-64")
            .is_ok_and(|c| c.get_int().is_some_and(|d| d == -64)));
        assert!(parse_str("123.4")
            .is_ok_and(|c| c.get_real().is_some_and(|d| d == 123.4)));
        assert!(parse_str("1.4e-8")
            .is_ok_and(|c| c.get_real().is_some_and(|d| d == 1.4e-8)));
        assert!(parse_str("null").is_ok_and(|c| c.is_null()));
        assert!(parse_str("true")
            .is_ok_and(|c| c.get_bool().is_some_and(|d| d == true)));
        assert!(parse_str("false")
            .is_ok_and(|c| c.get_bool().is_some_and(|d| d == false)));
        assert!(parse_str("\"false\"")
            .is_ok_and(|c| c.get_string().is_some_and(|d| d == "false".to_owned())));

        Ok(())
    }
    
    #[test]
    fn test_escaped_string() -> Result<(), Box<dyn core::error::Error>> {
        assert!(parse_str("\"Someone said \\\"The brown fox jumps over the lazy dog.\\\"\"")
            .is_ok_and(|c| c.get_string().is_some_and(|d|{
                d == "Someone said \"The brown fox jumps over the lazy dog.\""
            })));
        assert!(parse_str("\"Encoding new line\\ncan be done as well. This is how\\t we do it.\"")
            .is_ok_and(|c| c.get_string().is_some_and(|d|{
                d == "Encoding new line\ncan be done as well. This is how\t we do it."
            })));
        assert!(parse_str("\"Encoding new line\\ncan be done as well. This is how\\a we do it.\"")
            .is_ok_and(|c| c.get_string().is_some_and(|d|{
                d == "Encoding new line\ncan be done as well. This is how\\a we do it."
            })));
        Ok(())
    }

    #[test]
    fn test_object() -> Result<(), Box<dyn core::error::Error>> {
        let a = parse_str(
            r#"{
            "tell": "me",
            "where": 123.98,
            "you": 1.9e2,
            "are": [
                1,2,3,4,5,6,7,8,9,10000.000987,2.99e-7
            ],
            "i'll": {
                "come":  "for",
                "you": [
                    "and",
                    { "parse": "json" },
                    true,
                    false,
                    { "eof": null }
                ]
            }
        }"#,
        )?;

        assert_eq!(a["tell"].get_string().unwrap(), "me");

        assert_eq!(a["are"][9].get_real().unwrap(), 10000.000987);
        assert_eq!(a["are"][2].get_uint().unwrap(), 3);
        assert_eq!(a["are"][10].get_real().unwrap(), 2.99e-7);

        assert_eq!(a["i'll"]["you"][0].get_string().unwrap(), "and");
        assert_eq!(a["i'll"]["you"][1]["parse"].get_string().unwrap(), "json");

        assert_eq!(a["i'll"]["you"][1]["parser"].get_string(), None);

        assert_eq!(a["i'll"]["you"][2].get_bool().unwrap(), true);
        Ok(())
    }

    #[test]
    fn test_empty() -> Result<(), Box<dyn core::error::Error>> {
        assert!(parse_str("{}").is_ok_and(|c| c.is_object() && c.len() == 0));
        assert!(parse_str("[]").is_ok_and(|c| c.is_array() && c.len() == 0));
        assert!(parse_str("[[]").is_err());

        Ok(())
    }
}
