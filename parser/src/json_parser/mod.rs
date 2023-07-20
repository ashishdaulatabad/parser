pub mod parser;

#[macro_export]
macro_rules! object {
    ([$($elem:tt),*]) => {{
        use crate::common::container::Container;
        let mut arr: Vec<Container> = Vec::new();
        $(
            arr.push(Container::Str($elem.to_owned()));
        )*
        Container::Array(arr)
    }};
    ($str:expr) => {{
        json_parser::parser::parse_str($str).unwrap()
    }};
    ($($key:tt : $value:tt),*) => {{
        use std::collections::HashMap;
        use crate::common::container::Container;
        let mut mp: HashMap<String, Container> = HashMap::new();
        $(
            mp.insert($key.to_owned(), Container::Str($value));
        )*
        Container::Object(mp)
    }};
}
