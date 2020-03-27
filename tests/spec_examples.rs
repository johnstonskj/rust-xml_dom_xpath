use xml_dom_xpath::xpath1::model::*;

//
// From https://www.w3.org/TR/xpath-10/#location-paths
//

#[test]
fn test_spec_location_path_examples_01() {
    assert_eq!(Step::child_elements("para").to_string(), "child::para");
}

#[test]
fn test_spec_location_path_examples_02() {
    assert_eq!(Step::all_child_elements().to_string(), "child::*");
}

#[test]
fn test_spec_location_path_examples_03() {
    assert_eq!(Step::all_child_text().to_string(), "child::text()");
}

#[test]
fn test_spec_location_path_examples_04() {
    assert_eq!(Step::all_children().to_string(), "child::node()");
}

#[test]
fn test_spec_location_path_examples_05() {
    assert_eq!(Step::attributes("name").to_string(), "attribute::name");
}

#[test]
fn test_spec_location_path_examples_06() {
    assert_eq!(Step::all_attributes().to_string(), "attribute::*");
}

#[test]
fn test_spec_location_path_examples_07() {
    assert_eq!(
        Step::descendant_elements("para").to_string(),
        "descendant::para"
    );
}

#[test]
fn test_spec_location_path_examples_08() {
    assert_eq!(
        Step::new(AxisSpecifier::Ancestor, NodeTest::Named("div".to_string())).to_string(),
        "ancestor::div"
    );
}

#[test]
fn test_spec_location_path_examples_09() {
    assert_eq!(
        Step::ancestor_or_self_elements("div").to_string(),
        "ancestor-or-self::div"
    );
}

#[test]
fn test_spec_location_path_examples_10() {
    assert_eq!(
        Step::descendant_or_self_elements("para").to_string(),
        "descendant-or-self::para"
    );
}

#[test]
fn test_spec_location_path_examples_11() {
    assert_eq!(Step::self_elements("para").to_string(), "self::para");
}

#[test]
fn test_spec_location_path_examples_12() {
    let mut path = LocationPath::new();
    path.child_elements("chapter").descendant_elements("para");
    assert_eq!(path.to_string(), "child::chapter/descendant::para");
}

#[test]
fn test_spec_location_path_examples_13() {
    let mut path = LocationPath::default();
    path.all_child_elements().child_elements("para");
    assert_eq!(path.to_string(), "child::*/child::para");
}

#[test]
fn test_spec_location_path_examples_14() {
    let path = LocationPath::root();
    assert_eq!(path.to_string(), "/");
}

#[test]
fn test_spec_location_path_examples_15() {
    let mut path = LocationPath::root();
    path.descendant_elements("para");
    assert_eq!(path.to_string(), "/descendant::para");
}

#[test]
fn test_spec_location_path_examples_16() {
    let mut path = LocationPath::root();
    path.descendant_elements("olist").child_elements("item");
    assert_eq!(path.to_string(), "/descendant::olist/child::item");
}

#[test]
fn test_spec_location_path_examples_17() {
    let mut path = LocationPath::default();
    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(1),
    ));

    path.append(step);
    assert_eq!(path.to_string(), "child::para[position() = 1]");
}

#[test]
fn test_spec_location_path_examples_18() {
    let mut path = LocationPath::default();
    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::function("last"),
    ));

    path.append(step);
    assert_eq!(path.to_string(), "child::para[position() = last()]");
}

#[test]
fn test_spec_location_path_examples_19() {
    let mut path = LocationPath::default();
    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::subtract(Predicate::function("last"), Predicate::integer(1)),
    ));

    path.append(step);
    assert_eq!(path.to_string(), "child::para[position() = last() - 1]");
}

#[test]
fn test_spec_location_path_examples_20() {
    let mut path = LocationPath::default();
    let mut step = Step::child_elements("para");

    step.append(Predicate::gt(
        Predicate::function("position"),
        Predicate::integer(1),
    ));

    path.append(step);
    assert_eq!(path.to_string(), "child::para[position() > 1]");
}

#[test]
fn test_spec_location_path_examples_xx() {
    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        Predicate::select(
            AxisSpecifier::Attribute,
            NodeTest::Named("type".to_string()),
        ),
        Predicate::literal("warning"),
    ))
    .append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(5),
    ));

    let path = LocationPath::new_with(step);
    assert_eq!(
        path.to_string(),
        "child::para[attribute::type = 'warning'][position() = 5]"
    );
}
