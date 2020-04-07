/*!
An implementation of W3C XML Path Language (XPath), over the [Document Object Model Core, Level
2](https://www.w3.org/TR/DOM-Level-2-Core).

Each module within this crate implements a version of the XPath language, the interface to each
remains the same, an expression parser, an in-memory model, and an evaluator that applies the
in-memory model to a set of nodes from a DOM model. Proving access to the in-memory model allows
clients to build these structures without having to parse text for common operations.

# Example

```rust,ignore
pub use xml_dom::parser::read_xml;
pub use xml_dom_xpath::xpath1::evaluate_path;

const TEST_XML: &str = r##"<?xml version="1.0"?>
<book xml:id="A">
  <chapter xml:id="B">
    <section xml:id="C">
      This is the first section of chapter 1.
    </section>
    <section xml:id="D" incomplete="true">
    </section>
  </chapter>
  <chapter xml:id="E">
    <section xml:id="F">
    </section>
    <section xml:id="G">
      <sub-section xml:id="H">
      </sub-section>
      <sub-section xml:id="I">
      </sub-section>
      <sub-section xml:id="J">
      </sub-section>
    </section>
  </chapter>
  <chapter xml:id="K">
    <section xml:id="L">
    </section>
    <section xml:id="M" incomplete="true">
    </section>
  </chapter>
  <chapter xml:id="N" incomplete="true">
  </chapter>
</book>"##;
let document_node = read_xml(TEST_XML).unwrap();

let result = evaluate_path("descendent-or-self::chapter", &[document_node]);
assert!(result.is_ok());
let result = result.unwrap();
```

*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest_derive;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod xpath1;
