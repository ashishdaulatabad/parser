use core::result::Result;

use crate::common::error::Error;
use crate::json_parser::parser::parse_str;
mod tests {
    use super::*;
    #[test]
    fn test_object() -> Result<(), Error> {
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
}
