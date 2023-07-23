mod common;
mod json_parser;

use json_parser::parser::parse_str;

fn main() {
    let pr = parse_str(r#"{
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
    }"#);
    let object: &mut crate::common::container::Container = &mut pr.unwrap();
    println!("{}", object);
    println!("{}", object["tell"]);
    object["tell"] = crate::common::container::Container::Decimal(12233.2);
    println!("{}", object["tell"]);

    let v = object!(r#"[false, 1.2344e-1] "#);
    println!("{}", v);
    let another = object!(
        [
            "some",
            "element",
            "are"
        ]
    );

    println!("{}", another);
}
