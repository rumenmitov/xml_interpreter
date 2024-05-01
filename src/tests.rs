#[cfg(test)]
use super::*;

#[test]
fn test_attribute_parsing() {
    let attribute = Attribute {
	key: String::from("name"),
	value: Some(String::from("jester")),
    };

    assert_eq!(Ok(AttributeEnding::Unfinished((attribute.clone(), ""))), Attribute::parse("name=jester "));
    assert_eq!(Ok(AttributeEnding::SelfClosing((attribute.clone(), ""))), Attribute::parse("name=jester/>"));
    assert_eq!(Ok(AttributeEnding::RequiresClosing((attribute, ""))), Attribute::parse("name=jester>"));
    assert_eq!(Ok(AttributeEnding::None), Attribute::parse(""));
}


#[test]
fn test_element_parsing() {
    let element = Element {
	id: 0,
	name: String::from("root"),
	attributes: Vec::new(),
	parent_id: None,
	children: Vec::new()
    };

    assert_eq!(Ok(ElementState::Opening((element.clone(), "</root>"))), Element::parse(0, "<root></root>"));
    assert_eq!(Ok(ElementState::Closing((element.clone(), ""))), Element::parse(0, "</root>"));
    assert_eq!(Ok(ElementState::SelfClosing((element, ""))), Element::parse(0, "<root />"));
    assert_eq!(Ok(ElementState::None), Element::parse(0, "<>"));
}


#[test]
fn test_element_tree() {
    let solution = ElementTree {
	root_id: 0,
	elements: vec![
	    Element {
		id: 0,
		name: String::from("root"),
		attributes: Vec::new(),
		parent_id: None,
		children: Vec::from([1]),
	    },

	    Element {
		id: 1,
		name: String::from("text"),
		attributes: vec![
		    Attribute {
			key: String::from("width"),
			value: Some(String::from("5")),
		    }
		],
		parent_id: Some(0),
		children: Vec::from([2]),
	    },

	    Element {
		id: 2,
		name: String::from("img"),
		attributes: Vec::new(),
		parent_id: Some(1),
		children: Vec::new(),
	    }
	]
    };

    assert_eq!(Ok(solution), ElementTree::new("<root><text width=5><img /></text></root>"));
}
