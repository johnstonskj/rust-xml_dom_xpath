/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::mem;
use std::sync::Once;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Bool,
    Number,
    String,
    Object,
    NodeSet,
}

#[derive(Clone, Debug)]
pub struct Argument {
    name: String,
    data_type: DataType,
    required: bool,
}

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    arguments: Vec<Argument>,
    result_type: DataType,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn is_function(name: &str) -> bool {
    function_map().contains_key(name)
}

pub fn get_function(name: &str) -> Option<Function> {
    let functions = function_map();
    functions.get(name).map(|f| f.clone())
}

pub fn required_functions() -> Vec<Function> {
    let functions = function_map();
    functions.values().cloned().collect()
}

fn function_map() -> HashMap<String, Function> {
    static mut FUNCTIONS: *const HashMap<String, Function> = 0 as *const HashMap<String, Function>;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            let all_functions = vec![
                // 4.1 Node Set Functions
                Function::new("last", &[], DataType::Number),
                Function::new("position", &[], DataType::Number),
                Function::new_from(
                    "count",
                    &[("node-set", DataType::NodeSet, true)],
                    DataType::Number,
                ),
                Function::new_from(
                    "id",
                    &[("object", DataType::Object, true)],
                    DataType::NodeSet,
                ),
                Function::new_from(
                    "local-name",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                Function::new_from(
                    "namespace-uri",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                Function::new_from(
                    "name",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                // 4.2 String Functions
                Function::new_from(
                    "string",
                    &[("object", DataType::Object, false)],
                    DataType::String,
                ),
                Function::new_from(
                    "concat",
                    &[
                        ("string-1", DataType::String, true),
                        ("string-2", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "starts-with",
                    &[
                        ("string", DataType::String, true),
                        ("test-prefix", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "contains",
                    &[
                        ("string", DataType::String, true),
                        ("test-in", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "substring-before",
                    &[
                        ("string", DataType::String, true),
                        ("split-at", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "substring-after",
                    &[
                        ("string", DataType::String, true),
                        ("split-at", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "substring",
                    &[
                        ("string", DataType::String, true),
                        ("start", DataType::Number, true),
                        ("length", DataType::Number, false),
                    ],
                    DataType::String,
                ),
                Function::new_from(
                    "string-length",
                    &[("string", DataType::String, false)],
                    DataType::String,
                ),
                Function::new_from(
                    "normalize-space",
                    &[("string", DataType::String, false)],
                    DataType::String,
                ),
                Function::new_from(
                    "translate",
                    &[
                        ("string", DataType::String, true),
                        ("replace", DataType::String, true),
                        ("with", DataType::String, true),
                    ],
                    DataType::String,
                ),
                // 4.3 Boolean Functions
                Function::new_from(
                    "boolean",
                    &[("object", DataType::Object, true)],
                    DataType::Bool,
                ),
                Function::new_from("not", &[("value", DataType::Bool, true)], DataType::Bool),
                Function::new_from("true", &[], DataType::Bool),
                Function::new_from("false", &[], DataType::Bool),
                Function::new_from(
                    "lang",
                    &[("string", DataType::String, true)],
                    DataType::Bool,
                ),
                // 4.4 Number Functions
                Function::new_from(
                    "number",
                    &[("object", DataType::Object, true)],
                    DataType::Number,
                ),
                Function::new_from(
                    "sum",
                    &[("node-set", DataType::NodeSet, true)],
                    DataType::Number,
                ),
                Function::new_from(
                    "floor",
                    &[("number", DataType::Number, true)],
                    DataType::Number,
                ),
                Function::new_from(
                    "ceiling",
                    &[("number", DataType::Number, true)],
                    DataType::Number,
                ),
                Function::new_from(
                    "round",
                    &[("number", DataType::Number, true)],
                    DataType::Number,
                ),
            ];
            let all_functions: HashMap<String, Function> = all_functions
                .iter()
                .map(|f| (f.name.clone(), f.clone()))
                .collect();
            FUNCTIONS = mem::transmute(Box::new(all_functions));
        });
        (*FUNCTIONS).clone()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Bool => "bool",
                DataType::Number => "Num",
                DataType::String => "&str",
                DataType::Object => "Object",
                DataType::NodeSet => "NodeSet",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}: {}",
            self.name,
            if self.required {
                self.data_type.to_string()
            } else {
                format!("Option<{}>", self.data_type)
            }
        )
    }
}

impl Argument {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self::new_from(name, data_type, true)
    }

    pub fn new_optional(name: &str, data_type: DataType) -> Self {
        Self::new_from(name, data_type, false)
    }

    pub(crate) fn new_from(name: &str, data_type: DataType, required: bool) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            required,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "fn {}({}) -> {};",
            self.name,
            self.arguments
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.result_type
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Function {
    pub fn new(name: &str, arguments: &[Argument], result_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            arguments: arguments.to_vec(),
            result_type,
        }
    }
    pub(crate) fn new_from(
        name: &str,
        arguments: &[(&str, DataType, bool)],
        result_type: DataType,
    ) -> Self {
        Self {
            name: name.to_string(),
            arguments: arguments
                .iter()
                .map(|(n, t, r)| Argument::new_from(n, t.clone(), *r))
                .collect(),
            result_type,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let functions = required_functions();
        for function in functions {
            println!("{}", function);
        }
    }
}
