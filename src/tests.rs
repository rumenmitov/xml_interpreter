#[cfg(test)]
use super::*;

#[test]
fn test_attribute_parsing() {
    let mut element = Element {
	id: 0,
	name: String::new(),
	attributes: Vec::new(),
	parent_id: None,
	children: VecDeque::new(),
	children_stack: Vec::new(),
    };

    assert_eq!(Ok(AttributeEnding::Unfinished("")), Attribute::parse(&mut element, "name=jester "));
    assert_eq!(Ok(AttributeEnding::SelfClosing("")), Attribute::parse(&mut element, "name=jester/>"));
    assert_eq!(Ok(AttributeEnding::RequiresClosing("")), Attribute::parse(&mut element, "name=jester>"));
    
}


#[test]
fn test_element_tree() {

    let solution = ElementTree {
	root_id: 0,
	elements_table: vec![
	    Element {
		id: 0,
		name: String::from("root"),
		attributes: vec![
		    Attribute {
			key: String::new(),
			value: None,
		    }
		],
		parent_id: None,
		children: VecDeque::from([1]),
		children_stack: Vec::new(),
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
		children: VecDeque::from([2]),
		children_stack: Vec::new(),
	    },

	    Element {
		id: 2,
		name: String::from("img"),
		attributes: vec![
		    Attribute {
			key: String::new(),
			value: None
		    }
		],
		parent_id: Some(1),
		children: VecDeque::new(),
		children_stack: Vec::new(),
	    }
	]
    };

    let et = ElementTree::new("<root><text width=5><img /></text></root>").unwrap();
    assert_eq!(solution, et);
    assert_eq!("root", et.elements_table[0].name);
    assert_eq!("text", et.elements_table[1].name);
    assert_eq!("img", et.elements_table[2].name);
}
