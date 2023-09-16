pub mod parser;
pub mod test;

#[macro_export]
macro_rules! object {
    ([$($elem:tt),*]) => {{
        use $crate::common::container::Container;
        Container::Array(vec![$( Container::String($elem.to_owned()) ),*])
    }};
    ($str:expr) => {{
        json_parser::parser::parse_str($str).unwrap()
    }};
    ($($key:tt : $value:tt),*) => {{
        use std::collections::HashMap;
        use $crate::common::container::Container;
        let mut mp: HashMap<String, Container> = HashMap::new();
        $(
            mp.insert($key.to_owned(), Container::String($value));
        )*
        Container::Object(mp)
    }};
}
