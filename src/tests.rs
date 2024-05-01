#[cfg(test)]
use super::*;

#[test]
fn test_attribute_parsing() {
    let attribute_solution = Attribute {
	key: String::from("name"),
	value: Some(String::from("jester")),
    };

    assert_eq!(Ok(AttributeEnding::Unfinished((attribute_solution.clone(), ""))), Attribute::parse("name=jester "));
    assert_eq!(Ok(AttributeEnding::SelfClosing((attribute_solution.clone(), ""))), Attribute::parse("name=jester/>"));
    assert_eq!(Ok(AttributeEnding::RequiresClosing((attribute_solution, ""))), Attribute::parse("name=jester>"));
    assert_eq!(Ok(AttributeEnding::None), Attribute::parse(""));
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

    let element_tree = ElementTree::new("<root><text width=5><img /></text></root>").unwrap();
    assert_eq!(solution, element_tree);
}
