use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};
use std::collections::{HashMap, HashSet};

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
/// let mut str: Container = Container::String("here".to_string());
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
/// object_container.insert("key1".to_string(), Container::String("hello".to_string()));
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
    String(String),
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
            Self::String(element) => Self::String(element.to_owned()),
            Self::Array(array) => Self::Array(array.clone()),
            Self::Object(object) => Self::Object(object.clone()),
            Self::Set(set) => Self::Set(set.clone()),
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
            Self::String(v) => v.hash(s),
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

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(this), Self::Number(other)) => this == other,
            (Self::Unsigned(this), Self::Unsigned(other)) => this == other,
            (Self::Decimal(this), Self::Decimal(other)) => this == other,
            (Self::Boolean(this), Self::Boolean(other)) => this == other,
            (Self::String(this), Self::String(other)) => this == other,
            (Self::Array(arr), Self::Array(oarr)) => {
                arr.len() == oarr.len()
                    && arr.iter().zip(oarr).all(|(a, b)| a == b)
            }
            (Self::Set(set), Self::Set(oset)) => {
                (set.len() == oset.len())
                    && set.iter().all(|v| oset.get(v) == Some(v))
            }
            (Self::Object(map), Self::Object(omap)) => {
                (map.len() == omap.len())
                    && map.iter().all(|(k, v)| omap.get(k) == Some(v))
            }
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Container {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.dump_object(true, 4, 1))
    }
}

#[allow(unused)]
/// To do: Implement index
impl Container {
    /// Returned New Object
    #[inline(always)]
    pub fn new_object() -> Self {
        Self::Object(HashMap::new())
    }

    /// Returns New Array Object
    #[inline(always)]
    pub fn new_array() -> Self {
        Self::Array(Vec::new())
    }

    /// Returns new set
    #[inline(always)]
    pub fn new_set() -> Self {
        Self::Set(HashSet::new())
    }

    /// Array: Push an item into array or an element into set:
    ///
    /// Returns `false` if element cannot be added in container
    /// Permissible for array type only
    pub fn push(&mut self, val: Self) -> bool {
        match self {
            // Array push
            Self::Array(value) => {
                value.push(val);
                true
            }
            Self::Set(value) => value.insert(val),
            _ => false,
        }
    }

    /// Insert/Replaces key value pair into Object, where a key is `&str` literal
    ///
    /// Returns `true` if success, else `false`.
    pub fn insert_str(&mut self, key: &str, val: Self) -> bool {
        match self {
            Self::Object(map) => map.insert(key.to_owned(), val).is_some(),
            _ => false,
        }
    }

    /// Dump value to a string.
    pub fn dump_object(
        &self,
        indent: bool,
        indent_size: usize,
        depth: usize,
    ) -> String {
        match self {
            Self::Array(value) => {
                if value.is_empty() {
                    "[]".to_owned()
                } else if !indent {
                    "[".to_owned()
                        + &value
                            .iter()
                            .map(|e| {
                                e.dump_object(indent, indent_size, depth + 1)
                            })
                            .collect::<Vec<String>>()
                            .join(",")
                        + "]"
                } else {
                    let wspace = " ".repeat((depth - 1) * indent_size);
                    let space = " ".repeat(depth * indent_size);

                    "[\n".to_owned()
                        + &value
                            .iter()
                            .map(|e| {
                                space.to_owned()
                                    + &e.dump_object(
                                        indent,
                                        indent_size,
                                        depth + 1,
                                    )
                            })
                            .collect::<Vec<String>>()
                            .join(",\n")
                        + "\n"
                        + &wspace
                        + "]"
                }
            }
            Self::Object(map) => {
                if map.is_empty() {
                    "{}".to_owned()
                } else if !indent {
                    "{".to_owned()
                        + &map
                            .iter()
                            .map(|(k, v)| {
                                format!("{:?}", k)
                                    + &v.dump_object(
                                        indent,
                                        indent_size,
                                        depth + 1,
                                    )
                            })
                            .collect::<Vec<String>>()
                            .join(",")
                        + "}"
                } else {
                    let wspace = " ".repeat((depth - 1) * indent_size);
                    let space = " ".repeat(depth * indent_size);

                    "{\n".to_owned()
                        + &map
                            .iter()
                            .map(|(k, v)| {
                                space.to_owned()
                                    + &format!("{:?}: ", k)
                                    + &v.dump_object(
                                        indent,
                                        indent_size,
                                        depth + 1,
                                    )
                            })
                            .collect::<Vec<String>>()
                            .join(",\n")
                        + "\n"
                        + &wspace
                        + "}"
                }
            }
            Self::Set(value) => {
                if value.is_empty() {
                    "()".to_owned()
                } else if !indent {
                    "(".to_owned()
                        + &value
                            .iter()
                            .map(|e| {
                                e.dump_object(indent, indent_size, depth + 1)
                            })
                            .collect::<Vec<String>>()
                            .join(",")
                        + ")"
                } else {
                    let wspace = " ".repeat((depth - 1) * indent_size);
                    let space = " ".repeat(depth * indent_size);

                    "(\n".to_owned()
                        + &value
                            .iter()
                            .map(|e| {
                                space.to_owned()
                                    + &e.dump_object(
                                        indent,
                                        indent_size,
                                        depth + 1,
                                    )
                            })
                            .collect::<Vec<String>>()
                            .join(",\n")
                        + "\n"
                        + &wspace
                        + ")"
                }
            }
            Self::Number(value) => value.to_string(),
            Self::Unsigned(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Decimal(value) => value.to_string(),
            Self::String(value) => format!("{:?}", value),
            Self::Null => "null".to_owned(),
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::String(value) => Some(value.to_owned()),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(value) => Some(value.to_owned()),
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

    define_type_checks!(String, is_str);

    define_type_checks!(Object, is_object);

    define_type_checks!(Set, is_set);

    pub fn is_null(&self) -> bool {
        *self == Self::Null
    }

    /// Returns the length of an object
    pub fn len(&self) -> usize {
        match self {
            Self::Array(value) => value.len(),
            Self::Object(value) => value.len(),
            Self::Set(value) => value.len(),
            Self::String(value) => value.len(),
            _ => 1,
        }
    }
}

impl Index<usize> for Container {
    type Output = Self;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Array(value) => {
                if value.len() > idx {
                    value.get(idx).unwrap()
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
                self.insert_str(idx.as_str(), Self::Null);
                &mut self[idx]
            }
        }
    }
}

impl IndexMut<&str> for Container {
    fn index_mut(&mut self, idx: &str) -> &mut Self {
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
                self.insert_str(idx, Self::Null);
                &mut self[idx]
            }
        }
    }
}
