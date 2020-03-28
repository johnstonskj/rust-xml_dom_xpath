use xml_dom_xpath::xpath1::model::*;

macro_rules! select {
    ($axis:ident, $test:ident) => {
        Predicate::select(AxisSpecifier::$axis, NodeTest::$test)
    };
    ($axis:ident named $name:expr) => {
        Predicate::select(AxisSpecifier::$axis, NodeTest::Named($name.to_string()))
    };
}

// ------------------------------------------------------------------------------------------------
// From https://www.w3.org/TR/xpath-10/#location-paths
// ------------------------------------------------------------------------------------------------

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
fn test_spec_location_path_examples_21() {
    let mut path = LocationPath::default();

    let mut step = Step::following_sibling_elements("chapter");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(1),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "following-sibling::chapter[position() = 1]"
    );
}

#[test]
fn test_spec_location_path_examples_22() {
    let mut path = LocationPath::default();

    let mut step = Step::preceding_sibling_elements("chapter");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(1),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "preceding-sibling::chapter[position() = 1]"
    );
}

#[test]
fn test_spec_location_path_examples_23() {
    let mut path = LocationPath::root();

    let mut step = Step::descendant_elements("figure");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(42),
    ));

    path.append(step);
    assert_eq!(path.to_string(), "/descendant::figure[position() = 42]");
}

#[test]
fn test_spec_location_path_examples_24() {
    let mut path = LocationPath::root();

    path.append(Step::child_elements("doc"));

    let mut step = Step::child_elements("chapter");
    path.append(
        step.append(Predicate::eq(
            Predicate::function("position"),
            Predicate::integer(5),
        ))
        .to_owned(),
    );

    let mut step = Step::child_elements("section");
    path.append(
        step.append(Predicate::eq(
            Predicate::function("position"),
            Predicate::integer(2),
        ))
        .to_owned(),
    );

    assert_eq!(
        path.to_string(),
        "/child::doc/child::chapter[position() = 5]/child::section[position() = 2]"
    );
}

#[test]
fn test_spec_location_path_examples_25() {
    let mut path = LocationPath::root();

    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        select!(Attribute named "type"),
        Predicate::literal("warning"),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "/child::para[attribute::type = 'warning']"
    );
}

#[test]
fn test_spec_location_path_examples_26() {
    let mut path = LocationPath::default();

    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        select!(Attribute named "type"),
        Predicate::literal("warning"),
    ));
    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(5),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "child::para[attribute::type = 'warning'][position() = 5]"
    );
}

#[test]
fn test_spec_location_path_examples_27() {
    let mut path = LocationPath::default();

    let mut step = Step::child_elements("para");

    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::integer(5),
    ));
    step.append(Predicate::eq(
        select!(Attribute named "type"),
        Predicate::literal("warning"),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "child::para[position() = 5][attribute::type = 'warning']"
    );
}

#[test]
fn test_spec_location_path_examples_28() {
    let mut path = LocationPath::default();

    let mut step = Step::child_elements("chapter");

    step.append(Predicate::eq(
        select!(Child named "title"),
        Predicate::literal("Introduction"),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "child::chapter[child::title = 'Introduction']"
    );
}

#[test]
fn test_spec_location_path_examples_29() {
    let mut path = LocationPath::default();

    let mut step = Step::child_elements("chapter");

    step.append(select!(Child named "title"));

    path.append(step);
    assert_eq!(path.to_string(), "child::chapter[child::title]");
}

#[test]
fn test_spec_location_path_examples_30() {
    let mut path = LocationPath::default();

    let mut step = Step::all_child_elements();

    step.append(Predicate::or(
        select!(SelfNode named "chapter"),
        select!(SelfNode named "appendix"),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "child::*[self::chapter or self::appendix]"
    );
}

#[test]
fn test_spec_location_path_examples_31() {
    let mut path = LocationPath::default();

    let mut step = Step::all_child_elements();

    step.append(Predicate::or(
        select!(SelfNode named "chapter"),
        select!(SelfNode named "appendix"),
    ));
    step.append(Predicate::eq(
        Predicate::function("position"),
        Predicate::function("last"),
    ));

    path.append(step);
    assert_eq!(
        path.to_string(),
        "child::*[self::chapter or self::appendix][position() = last()]"
    );
}
