/*!
Parse an XPath string into it's model form. The primary API is the [`read_str`](fn.read_str.html)
function.

# Specification

```ebnf
[1]  LocationPath          ::=  RelativeLocationPath
                                | AbsoluteLocationPath
[2]  AbsoluteLocationPath  ::=  '/' RelativeLocationPath?
                                | AbbreviatedAbsoluteLocationPath
[3]  RelativeLocationPath  ::=  Step
                                | RelativeLocationPath '/' Step
                                | AbbreviatedRelativeLocationPath

[4]  Step                  :=   AxisSpecifier NodeTest Predicate*
                                | AbbreviatedStep
[5]  AxisSpecifier         ::=  AxisName '::'
                                | AbbreviatedAxisSpecifier


[6]  AxisName              ::=  'ancestor'
                                | 'ancestor-or-self'
                                | 'attribute'
                                | 'child'
                                | 'descendant'
                                | 'descendant-or-self'
                                | 'following'
                                | 'following-sibling'
                                | 'namespace'
                                | 'parent'
                                | 'preceding'
                                | 'preceding-sibling'
                                | 'self'


[7]  NodeTest              ::=  NameTest
                                | NodeType '(' ')'
                                | 'processing-instruction' '(' Literal ')'


[8]  Predicate             ::=  '[' PredicateExpr ']'
[9]  PredicateExpr         ::=  Expr


[10]  AbbreviatedAbsoluteLocationPath  ::=  '//' RelativeLocationPath
[11]  AbbreviatedRelativeLocationPath  ::=  RelativeLocationPath '//' Step
[12]  AbbreviatedStep                  ::=  '.'  |  '..'
[13]  AbbreviatedAxisSpecifier         ::=  '@'?


[14]  Expr                 ::=  OrExpr
[15]  PrimaryExpr          ::=  VariableReference
                                | '(' Expr ')'
                                | Literal
                                | Number
                                | FunctionCall


[16]  FunctionCall         ::=  FunctionName '(' ( Argument ( ',' Argument )* )? ')'
[17]  Argument             ::=  Expr


[18]  UnionExpr            ::=  PathExpr
                                | UnionExpr '|' PathExpr
[19]  PathExpr             ::=   LocationPath
                                | FilterExpr
                                | FilterExpr '/' RelativeLocationPath
                                | FilterExpr '//' RelativeLocationPath
[20]  FilterExpr           ::=  PrimaryExpr
                                | FilterExpr Predicate


[21]  OrExpr               ::=   AndExpr
                                | OrExpr 'or' AndExpr
[22]  AndExpr              ::=  EqualityExpr
                                | AndExpr 'and' EqualityExpr
[23]  EqualityExpr         ::=   RelationalExpr
                                | EqualityExpr '=' RelationalExpr
                                | EqualityExpr '!=' RelationalExpr
[24]  RelationalExpr       ::=  AdditiveExpr
                                | RelationalExpr '<' AdditiveExpr
                                | RelationalExpr '>' AdditiveExpr
                                | RelationalExpr '<=' AdditiveExpr
                                | RelationalExpr '>=' AdditiveExpr


[25]  AdditiveExpr         ::=  MultiplicativeExpr
                                | AdditiveExpr '+' MultiplicativeExpr
                                | AdditiveExpr '-' MultiplicativeExpr
[26]  MultiplicativeExpr   ::=   UnaryExpr
                                | MultiplicativeExpr MultiplyOperator UnaryExpr
                                | MultiplicativeExpr 'div' UnaryExpr
                                | MultiplicativeExpr 'mod' UnaryExpr
[27]  UnaryExpr            ::=  UnionExpr
                                | '-' UnaryExpr


[28]  ExprToken            ::=  '(' | ')' | '[' | ']' | '.' | '..' | '@' | ',' | '::'
                                | NameTest
                                | NodeType
                                | Operator
                                | FunctionName
                                | AxisName
                                | Literal
                                | Number
                                | VariableReference
[29]  Literal             ::=   '"' [^"]* '"'
                                | "'" [^']* "'"
[30]  Number              ::=   Digits ('.' Digits?)?
                                | '.' Digits
[31]  Digits              ::=   [0-9]+
[32]  Operator            ::=   OperatorName
                                | MultiplyOperator
                                | '/' | '//' | '|' | '+' | '-' | '=' | '!=' | '<' | '<=' | '>' | '>='
[33]  OperatorName        ::=   'and' | 'or' | 'mod' | 'div'
[34]  MultiplyOperator    ::=   '*'
[35]  FunctionName        ::=   QName - NodeType
[36]  VariableReference   ::=   '$' QName
[37]  NameTest            ::=   '*'
                                | NCName ':' '*'
                                | QName
[38]  NodeType            ::=   'comment'
                                | 'text'
                                | 'processing-instruction'
                                | 'node'
[39]  ExprWhitespace      ::=   S
```

*/

use crate::xpath1::model::LocationPath;
use pest::error::Error;
use pest::Parser;
use pest_parser::{Rule, XPathParser};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    Parser(Error<Rule>),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn read_str(xpath_str: &str) -> Result<LocationPath, ParseError> {
    if xpath_str.is_empty() {
        return Err(ParseError::EmptyString);
    }
    let _path = XPathParser::parse(Rule::LocationPath, xpath_str)?
        .next()
        .unwrap();

    Err(ParseError::EmptyString)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseError::EmptyString => "The path string is empty".to_string(),
                ParseError::Parser(err) => format!("The parser failed. {:?}", err),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl std::error::Error for ParseError {}

// ------------------------------------------------------------------------------------------------

impl From<Error<Rule>> for ParseError {
    fn from(err: Error<Rule>) -> Self {
        Self::Parser(err)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod pest_parser;
