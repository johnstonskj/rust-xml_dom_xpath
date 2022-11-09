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

    fn test_spec_example(example: &str, description: &str, dump: bool) {
        println!("* {}", description);
        println!("  XPath: '{}'", example);
        let result = XPathParser::parse(Rule::LocationPath, example);
        if dump {
            println!("  Parsed: {:#?}", result);
        } else if let Err(err) = &result {
            println!("{}\n{:#?}", &err, &err);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_spec_example_01() {
        test_spec_example(
            "child::para",
            "selects the para element children of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_02() {
        test_spec_example(
            "child::*",
            "selects all element children of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_03() {
        test_spec_example(
            "child::text()",
            "selects all text node children of the context node",
            true,
        );
    }

    #[test]
    fn test_spec_example_04() {
        test_spec_example(
            "child::node()",
            "selects all the children of the context node, whatever their node type",
            false,
        );
    }

    #[test]
    fn test_spec_example_05() {
        test_spec_example(
            "attribute::name",
            "selects the name attribute of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_06() {
        test_spec_example(
            "attribute::*",
            "selects all the attributes of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_07() {
        test_spec_example(
            "descendant::para",
            "selects the para element descendants of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_08() {
        test_spec_example(
            "ancestor::div",
            "selects all div ancestors of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_09() {
        test_spec_example("ancestor-or-self::div", "selects the div ancestors of the context node and, if the context node is a div element, the context node as well", false);
    }

    #[test]
    fn test_spec_example_10() {
        test_spec_example("descendant-or-self::para", "selects the para element descendants of the context node and, if the context node is a para element, the context node as well", false);
    }

    #[test]
    fn test_spec_example_11() {
        test_spec_example(
            "self::para",
            "selects the context node if it is a para element, and otherwise selects nothing",
            false,
        );
    }

    #[test]
    fn test_spec_example_12() {
        test_spec_example("child::chapter/descendant::para", "selects the para element descendants of the chapter element children of the context node", false);
    }

    #[test]
    fn test_spec_example_13() {
        test_spec_example(
            "child::*/child::para",
            "selects all para grandchildren of the context node",
            true,
        );
    }

    #[test]
    fn test_spec_example_14() {
        test_spec_example("/", "selects the document root test_spec_example(which is always the parent of the document element)", false);
    }

    #[test]
    fn test_spec_example_15() {
        test_spec_example(
            "/descendant::para",
            "selects all the para elements in the same document as the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_16() {
        test_spec_example("/descendant::olist/child::item", "selects all the item elements that have an olist parent and that are in the same document as the context node", false);
    }

    #[test]
    fn test_spec_example_17() {
        test_spec_example(
            "child::para[position()=1]",
            "selects the first para child of the context node",
            true,
        );
    }

    #[test]
    fn test_spec_example_18() {
        test_spec_example(
            "child::para[position() = last()]",
            "selects the last para child of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_19() {
        test_spec_example(
            "child::para[position()=last()-1]",
            "selects the last but one para child of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_20() {
        test_spec_example("child::para[position()>1]", "selects all the para children of the context node other than the first para child of the context node", false);
    }

    #[test]
    fn test_spec_example_21() {
        test_spec_example(
            "following-sibling::chapter[position()=1]",
            "selects the next chapter sibling of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_22() {
        test_spec_example(
            "preceding-sibling::chapter[position()=1]",
            "selects the previous chapter sibling of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_23() {
        test_spec_example(
            "/descendant::figure[position()=42]",
            "selects the forty-second figure element in the document",
            false,
        );
    }

    #[test]
    fn test_spec_example_24() {
        test_spec_example(
            "/child::doc/child::chapter[position()=5]/child::section[position()=2]",
            "selects the second section of the fifth chapter of the doc document element",
            false,
        );
    }

    #[test]
    fn test_spec_example_25() {
        test_spec_example("child::para[attribute::type='warning']", "selects all para children of the context node that have a type attribute with value warning", false);
    }

    #[test]
    fn test_spec_example_26() {
        test_spec_example("child::para[attribute::type='warning'][position()=5]", "selects the fifth para child of the context node that has a type attribute with value warning", false);
    }

    #[test]
    fn test_spec_example_27() {
        test_spec_example("child::para[position()=5][attribute::type='warning']", "selects the fifth para child of the context node if that child has a type attribute with value warning", false);
    }

    #[test]
    fn test_spec_example_28() {
        test_spec_example("child::chapter[child::title='Introduction']", "selects the chapter children of the context node that have one or more title children with string-value equal to Introduction", false);
    }

    #[test]
    fn test_spec_example_29() {
        test_spec_example(
            "child::chapter[child::title]",
            "selects the chapter children of the context node that have one or more title children",
            false,
        );
    }

    #[test]
    fn test_spec_example_30() {
        test_spec_example(
            "child::*[self::chapter or self::appendix]",
            "selects the chapter and appendix children of the context node",
            false,
        );
    }

    #[test]
    fn test_spec_example_31() {
        test_spec_example(
            "child::*[self::chapter or self::appendix][position()=last()]",
            "selects the last chapter or appendix child of the context node",
            false,
        );
    }
}
