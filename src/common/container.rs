use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::option::Option::Some;

/// A Container that has ability to store different kind
/// of data at a time. This includes basic data types like
/// - Null
/// - Integer
/// - Unsigned Integer
/// - Real Number
/// - Truth/Fallacy (Boolean)
/// - String values (These are displayed in double inverted quotes)
///
/// ## Examples for basic types
/// ```
/// let storage: Container = Container::Decimal(2e9+76);
/// let mut str: Container = Container::Str("here".to_string());
/// println!("{}, and {}", str);          // prints "here" and 2000000076
/// ```
///
/// And combination of such like:
/// - Array (An expandable, randomly accessible list)
/// - Set (A HashSet that stores unique values)
/// - Object (A HashMap, that associates a string key with a value)
///
/// ```
/// let array_container: Container = Container::new_array();
/// array_container.push(Container::Boolean(true));
/// array_container.push(Container::Number(1<<32));
/// array_container.push(Container::Decimal(2.34));
///
/// let object_container: Container = Container::new_object();
/// object_container.insert("key1".to_string(), Container::Str("hello".to_string()));
///
/// array_container.push(object_container);
/// println!("{}", array_container); /// dumps [true,4294967296,2.34,{"key1":"hello"}] in pretty fashion
/// ```
/// Todo:
/// - [ ] Support Date and raw binary data type
///
#[derive(Debug)]
pub enum Container {
    /// Representing an object of null type
    Null,
    /// A 16 byte signed integer
    Number(i64),
    /// A 16 byte unsigned integer
    Unsigned(u64),
    /// An 8 byte real number
    Decimal(f64),
    /// boolean value
    Boolean(bool),
    /// String
    Str(String),
    /// Dynamic allocated that can store
    /// these containers in consecutive fashion
    /// of their insertion.
    Array(Vec<Container>),
    /// Set containing unique container elements
    /// identified by either
    ///
    /// - Value or
    /// - Values inside these elements
    Set(HashSet<Container>),
    /// Key value pair, where key is string
    /// and value can be any of these types
    Object(HashMap<String, Container>),
}

impl Clone for Container {
    /// Creates an exact clone of self.
    fn clone(&self) -> Self {
        match self {
            Self::Number(element) => Self::Number(*element),
            Self::Unsigned(element) => Self::Unsigned(*element),
            Self::Decimal(element) => Self::Decimal(*element),
            Self::Boolean(element) => Self::Boolean(*element),
            Self::Str(ref element) => Self::Str(element.to_owned()),
            Self::Array(ref array) => Self::Array(array.clone()),
            Self::Object(ref object) => Self::Object(object.clone()),
            Self::Set(ref set) => Self::Set(set.clone()),
            Self::Null => Self::Null,
        }
    }
}

impl Hash for Container {
    fn hash<H: Hasher>(&self, s: &mut H) {
        match self {
            Self::Number(v) => v.hash(s),
            Self::Unsigned(v) => v.hash(s),
            Self::Boolean(v) => v.hash(s),
            Self::Str(ref v) => v.hash(s),
            _ => (),
        }
    }
}

macro_rules! define_type_checks {
    ($gen_type:ident, $func:ident) => {
        pub fn $func(&self) -> bool {
            match self {
                Self::$gen_type(_) => true,
                _ => false,
            }
        }
    };
}

impl Eq for Container {}

// == operator declaration
impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(this), Self::Number(other)) => this == other,
            (Self::Unsigned(this), Self::Unsigned(other)) => this == other,
            (Self::Decimal(this), Self::Decimal(other)) => this == other,
            (Self::Boolean(this), Self::Boolean(other)) => this == other,
            (Self::Str(this), Self::Str(other)) => this == other,
            (Self::Array(arr), Self::Array(oarr)) => {
                arr.len() == oarr.len() && arr.iter().zip(oarr.iter()).all(|(a, b)| a == b)
            }
            (Self::Set(set), Self::Set(other_set)) => {
                (set.len() == other_set.len())
                    && set.iter().all(|value| other_set.get(value) == Some(value))
            }
            (Self::Object(map_object), Self::Object(other_map_object)) => {
                (map_object.len() == other_map_object.len())
                    && map_object
                        .iter()
                        .all(|(key, value)| other_map_object.get(key) == Some(value))
            }
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

// Display Object
impl fmt::Display for Container {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.dump_object(true, 4, ""))
    }
}

// To do: Implement index
impl Container {
    // Returned New Object
    #[inline(always)]
    pub fn new_object() -> Self {
        Self::Object(HashMap::new())
    }

    // Returns New Array Object
    #[inline(always)]
    pub fn new_array() -> Self {
        Self::Array(Vec::new())
    }

    // Returns new set
    #[inline(always)]
    pub fn new_set() -> Self {
        Self::Set(HashSet::new())
    }

    // Array: Push an item into array or an element into set:
    // Returns false if not inserted
    // Permissible for array type only
    pub fn push(&mut self, val: Self) -> bool {
        match self {
            // Array push
            Self::Array(ref mut value) => {
                value.push(val);
                true
            }
            Self::Set(ref mut value) => value.insert(val),
            _ => {
                println!("Error: The storage type should be of array or a set for pushing values.");
                false
            }
        }
    }

    // Insert/Replaces key value pair into Object
    // Returns true if success, else false.
    pub fn insert(&mut self, key: String, val: Self) -> bool {
        match self {
            Self::Object(map) => map.insert(key.to_owned(), val) != None,
            _ => {
                println!("Error: The storage should be of type Object");
                false
            }
        }
    }

    // Insert/Replaces key value pair into Object
    // Returns true if success, else false.
    pub fn insert_str(&mut self, key: &str, val: Self) -> bool {
        match self {
            Self::Object(map) => map.insert(key.to_owned(), val) != None,
            _ => {
                println!("Error: The storage should be of type Object");
                false
            }
        }
    }

    // Print the Stored value
    pub fn dump_object(&self, indent: bool, indent_size: u8, white_space: &str) -> String {
        match self {
            Self::Array(value) => {
                if !indent {
                    format!(
                        "[{}]",
                        value
                            .iter()
                            .map(|element| element.dump_object(indent, indent_size, white_space))
                            .enumerate().fold(String::from(""), |prev, (idx, curr)| {
                                prev + if idx == 0 { "" } else { "," } + &curr
                            })
                    )
                } else {
                    if value.len() == 0 {
                        "[]".to_owned()
                    } else {
                        let space = white_space.to_owned() + 
                            (0..indent_size).map(|_| ' ').collect::<String>().as_str();
                        format!(
                            "[\n{}\n{}]",
                            value
                                .iter()
                                .map(|element| format!(
                                    "{}{}", space, element.dump_object(indent, indent_size, &space)
                                )).enumerate().fold(String::from(""), |prev, (idx, curr)| {
                                    prev + if idx == 0 { "" } else { ",\n" } + &curr
                                }),
                            white_space
                        )
                    }
                }
            }
            Self::Object(map) => {
                if !indent {
                    format!(
                        "{{{}}}",
                        map.iter().map(|(key, val)| format!(
                            "{:?}:{}", key, val.dump_object(indent, indent_size, white_space)
                        )).enumerate().fold(String::from(""), |prev, (idx, curr)| {
                            prev + if idx == 0 { "" } else { "," } + &curr
                        })
                    )
                } else {
                    if map.len() == 0 {
                        "{}".to_owned()
                    } else {
                        let space = white_space.to_owned() + 
                            (0..indent_size).map(|_| ' ').collect::<String>().as_str();
                        // space += c;
                        format!(
                            "{{{}\n{}}}",
                            map.iter().map(|(key, val)| format!(
                                "{}{:?}: {}", space, key, val.dump_object(indent, indent_size, &space)
                            )).enumerate().fold(String::from(""), |prev, (idx, curr)| {
                                prev + if idx == 0 { "\n" } else { ",\n" } + &curr
                            }),
                            white_space
                        )
                    }
                }
            }
            Self::Set(value) => {
                if !indent {
                    format!(
                        "({})",
                        value
                            .iter()
                            .map(|element| element.dump_object(indent, indent_size, white_space))
                            .enumerate().fold(String::from(""), |prev, (idx, curr)| {
                                prev + if idx == 0 { "" } else { "," } + &curr
                            })
                    )
                } else {
                    if value.len() == 0 {
                        "()".to_owned()
                    } else {
                        let space = white_space.to_owned() + 
                            (0..indent_size).map(|_| ' ').collect::<String>().as_str();
                        format!(
                            "(\n{}\n{})",
                            value
                                .iter()
                                .map(|element| format!(
                                    "{}{}", space, element.dump_object(indent, indent_size, &space)
                                )).enumerate().fold(String::from(""), |prev, (idx, curr)| {
                                    prev + if idx == 0 { "" } else { ",\n" } + &curr
                                }),
                            white_space
                        )
                    }
                }
            }
            Self::Number(value) => value.to_string(),
            Self::Unsigned(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Decimal(value) => value.to_string(),
            Self::Str(ref value) => format!("{:?}", value),
            Self::Null => "null".to_owned(),
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::Str(value) => Some(value.to_owned()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::Str(value) => Some(value.to_owned()),
            _ => None,
        }
    }

    pub fn get_uint(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_int(&self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_real(&self) -> Option<f64> {
        match self {
            Self::Decimal(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    define_type_checks!(Number, is_number);

    define_type_checks!(Unsigned, is_unsigned);

    define_type_checks!(Decimal, is_decimal);

    define_type_checks!(Boolean, is_bool);

    define_type_checks!(Str, is_str);

    define_type_checks!(Object, is_object);

    define_type_checks!(Set, is_set);

    pub fn is_null(self) -> bool {
        self == Self::Null
    }

    /// Returns the length of an object
    pub fn len(&self) -> usize {
        match self {
            Self::Array(value) => value.len(),
            Self::Object(value) => value.len(),
            Self::Set(value) => value.len(),
            Self::Str(value) => value.len(),
            _ => 1,
        }
    }
}

impl Index<usize> for Container {
    type Output = Self;
    // Returns the value given the index (usize).
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Array(value) => {
                if value.len() > idx {
                    &value.get(idx).unwrap()
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl Index<String> for Container {
    type Output = Self;
    // Returns the value given the string index
    fn index(&self, idx: String) -> &Self::Output {
        match self {
            Self::Object(value) => {
                if let Some(value) = value.get(&idx) {
                    value
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl Index<&str> for Container {
    type Output = Self;
    // Returns the value given the string index
    fn index(&self, idx: &str) -> &Self::Output {
        match self {
            Self::Object(value) => {
                if let Some(value) = value.get(&idx.to_owned()) {
                    value
                } else {
                    &Self::Null
                }
            }
            _ => &Self::Null,
        }
    }
}

impl IndexMut<usize> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, index: usize) -> &mut Self {
        match self {
            Self::Array(value) => {
                if value.len() > index {
                    &mut value[index]
                } else {
                    value.push(Self::Null);
                    value.last_mut().unwrap()
                }
            }
            _ => {
                // Log: Change into array warning
                *self = Self::new_array();
                self.push(Self::Null);
                &mut self[0]
            }
        }
    }
}

impl IndexMut<String> for Container {
    // Returns the value given the index (usize).
    fn index_mut(&mut self, idx: String) -> &mut Self {
        match self {
            Self::Object(value) => {
                if !value.contains_key(&idx) {
                    value.insert(idx.to_owned(), Self::Null);
                }
                value.get_mut(&idx).unwrap()
            }
            _ => {
                // Log: Change into array warning
                *self = Self::new_object();
                self.insert(idx.to_owned(), Self::Null);
                &mut self[idx]
            }
        }
    }
}

impl IndexMut<&str> for Container {
    // Returns the value given the index (usize).
    fn index_mut<'a>(&mut self, idx: &'a str) -> &mut Self {
        match self {
            Self::Object(value) => {
                let key = idx.to_owned();
                if !value.contains_key(&key) {
                    value.insert(key.to_owned(), Self::Null);
                }
                value.get_mut(&key).unwrap()
            }
            _ => {
                *self = Self::new_object();
                self.insert(idx.to_owned(), Self::Null);
                &mut self[idx]
            }
        }
    }
}
