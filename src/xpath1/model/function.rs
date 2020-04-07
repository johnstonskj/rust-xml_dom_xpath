use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::mem;
use std::sync::Once;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

#[allow(dead_code)]
pub fn get_function(name: &str) -> Option<Function> {
    let functions = function_map();
    functions.get(name).map(|f| f.clone())
}

#[allow(dead_code)]
pub fn required_functions() -> Vec<Function> {
    let functions = function_map();
    functions.values().cloned().collect()
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
    // pub fn new(name: &str, data_type: DataType) -> Self {
    //     Self::new_from(name, data_type, true)
    // }
    //
    // pub fn new_optional(name: &str, data_type: DataType) -> Self {
    //     Self::new_from(name, data_type, false)
    // }

    pub(crate) fn with(name: &str, data_type: DataType, required: bool) -> Self {
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
    pub fn with(name: &str, arguments: &[Argument], result_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            arguments: arguments.to_vec(),
            result_type,
        }
    }
    pub(crate) fn from_components(
        name: &str,
        arguments: &[(&str, DataType, bool)],
        result_type: DataType,
    ) -> Self {
        Self {
            name: name.to_string(),
            arguments: arguments
                .iter()
                .map(|(n, t, r)| Argument::with(n, t.clone(), *r))
                .collect(),
            result_type,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn function_map() -> HashMap<String, Function> {
    static mut FUNCTIONS: *const HashMap<String, Function> = 0 as *const HashMap<String, Function>;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            let all_functions = vec![
                // 4.1 Node Set Functions
                Function::with("last", &[], DataType::Number),
                Function::with("position", &[], DataType::Number),
                Function::from_components(
                    "count",
                    &[("node-set", DataType::NodeSet, true)],
                    DataType::Number,
                ),
                Function::from_components(
                    "id",
                    &[("object", DataType::Object, true)],
                    DataType::NodeSet,
                ),
                Function::from_components(
                    "local-name",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                Function::from_components(
                    "namespace-uri",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                Function::from_components(
                    "name",
                    &[("node-set?", DataType::NodeSet, false)],
                    DataType::String,
                ),
                // 4.2 String Functions
                Function::from_components(
                    "string",
                    &[("object", DataType::Object, false)],
                    DataType::String,
                ),
                Function::from_components(
                    "concat",
                    &[
                        ("string-1", DataType::String, true),
                        ("string-2", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "starts-with",
                    &[
                        ("string", DataType::String, true),
                        ("test-prefix", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "contains",
                    &[
                        ("string", DataType::String, true),
                        ("test-in", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "substring-before",
                    &[
                        ("string", DataType::String, true),
                        ("split-at", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "substring-after",
                    &[
                        ("string", DataType::String, true),
                        ("split-at", DataType::String, true),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "substring",
                    &[
                        ("string", DataType::String, true),
                        ("start", DataType::Number, true),
                        ("length", DataType::Number, false),
                    ],
                    DataType::String,
                ),
                Function::from_components(
                    "string-length",
                    &[("string", DataType::String, false)],
                    DataType::String,
                ),
                Function::from_components(
                    "normalize-space",
                    &[("string", DataType::String, false)],
                    DataType::String,
                ),
                Function::from_components(
                    "translate",
                    &[
                        ("string", DataType::String, true),
                        ("replace", DataType::String, true),
                        ("with", DataType::String, true),
                    ],
                    DataType::String,
                ),
                // 4.3 Boolean Functions
                Function::from_components(
                    "boolean",
                    &[("object", DataType::Object, true)],
                    DataType::Bool,
                ),
                Function::from_components(
                    "not",
                    &[("value", DataType::Bool, true)],
                    DataType::Bool,
                ),
                Function::from_components("true", &[], DataType::Bool),
                Function::from_components("false", &[], DataType::Bool),
                Function::from_components(
                    "lang",
                    &[("string", DataType::String, true)],
                    DataType::Bool,
                ),
                // 4.4 Number Functions
                Function::from_components(
                    "number",
                    &[("object", DataType::Object, true)],
                    DataType::Number,
                ),
                Function::from_components(
                    "sum",
                    &[("node-set", DataType::NodeSet, true)],
                    DataType::Number,
                ),
                Function::from_components(
                    "floor",
                    &[("number", DataType::Number, true)],
                    DataType::Number,
                ),
                Function::from_components(
                    "ceiling",
                    &[("number", DataType::Number, true)],
                    DataType::Number,
                ),
                Function::from_components(
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
