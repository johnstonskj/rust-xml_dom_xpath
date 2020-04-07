/*!
One-line description.

More detailed description, with

# Example

*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "xpath.pest"]
pub struct XPathParser;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

use pest::prec_climber::PrecClimber;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::Assoc::*;
        use pest::prec_climber::Operator;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(descendant, Left) | Operator::new(descendant_or_self, Left),
            Operator::new(multiply, Left) | Operator::new(modulus, Left) | Operator::new(div, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(lt, Left)
                | Operator::new(lteq, Left)
                | Operator::new(gt, Left)
                | Operator::new(gteq, Left),
            Operator::new(eq, Left) | Operator::new(neq, Left),
            Operator::new(and, Left) | Operator::new(or, Left),
        ])
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_spec_examples_parse() {
        let examples: &[(&str, &str, Option<&str>)] = &[
            ("child::para", "selects the para element children of the context node", None),
            ("child::*", "selects all element children of the context node", None),
            ("child::text()", "selects all text node children of the context node", None),
            ("child::node()", "selects all the children of the context node, whatever their node type", None),
            ("attribute::name", "selects the name attribute of the context node", None),
            ("attribute::*", "selects all the attributes of the context node", None),
            ("descendant::para", "selects the para element descendants of the context node", None),
            ("ancestor::div", "selects all div ancestors of the context node", None),
            ("ancestor-or-self::div", "selects the div ancestors of the context node and, if the context node is a div element, the context node as well", None),
            ("descendant-or-self::para", "selects the para element descendants of the context node and, if the context node is a para element, the context node as well", None),
            ("self::para", "selects the context node if it is a para element, and otherwise selects nothing", None),
            ("child::chapter/descendant::para", "selects the para element descendants of the chapter element children of the context node", None),
            ("child::*/child::para", "selects all para grandchildren of the context node", None),
            ("/", "selects the document root (which is always the parent of the document element)", None),
            ("/descendant::para", "selects all the para elements in the same document as the context node", None),
            ("/descendant::olist/child::item", "selects all the item elements that have an olist parent and that are in the same document as the context node", None),
            ("child::para[position()=1]", "selects the first para child of the context node", None),
            ("child::para[position()=last()]", "selects the last para child of the context node", None),
            ("child::para[position()=last()-1]", "selects the last but one para child of the context node", None),
            ("child::para[position()>1]", "selects all the para children of the context node other than the first para child of the context node", None),
            ("following-sibling::chapter[position()=1]", "selects the next chapter sibling of the context node", None),
            ("preceding-sibling::chapter[position()=1]", "selects the previous chapter sibling of the context node", None),
            ("/descendant::figure[position()=42]", "selects the forty-second figure element in the document", None),
            ("/child::doc/child::chapter[position()=5]/child::section[position()=2]", "selects the second section of the fifth chapter of the doc document element", None),
            ("child::para[attribute::type='warning']", "selects all para children of the context node that have a type attribute with value warning", None),
            ("child::para[attribute::type='warning'][position()=5]", "selects the fifth para child of the context node that has a type attribute with value warning", None),
            ("child::para[position()=5][attribute::type='warning']", "selects the fifth para child of the context node if that child has a type attribute with value warning", None),
            ("child::chapter[child::title='Introduction']", "selects the chapter children of the context node that have one or more title children with string-value equal to Introduction", None),
            ("child::chapter[child::title]", "selects the chapter children of the context node that have one or more title children", None),
            ("child::*[self::chapter or self::appendix]", "selects the chapter and appendix children of the context node", None),
            ("child::*[self::chapter or self::appendix][position()=last()]", "selects the last chapter or appendix child of the context node", None),
        ];
        for (example, description, _) in examples {
            println!("{}", example);
            assert!(XPathParser::parse(Rule::LocationPath, example).is_ok());
            println!("\t{}", description);
        }
    }
}
