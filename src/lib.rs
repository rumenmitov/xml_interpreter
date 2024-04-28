use std::collections::{VecDeque, HashSet};
use std::fmt;



type Id = usize;

#[derive(Debug, PartialEq)]
struct Attribute {
    key: String,
    value: Option<String>,
}

#[derive(Debug, PartialEq)]
enum AttributeEnding<'a> {
    Unfinished(&'a str),
    SelfClosing(&'a str),
    RequiresClosing(&'a str),
    None,
}

impl<'a> Attribute {
    fn parse(element :&mut Element, input :&'a str) -> Result<AttributeEnding<'a>, String> {
	let mut attribute = Attribute {
	    key: String::new(),
	    value: None
	};

	let trimmed_input = input.trim_start();

	let mut it_input = trimmed_input.char_indices();

	while let Some((i, ch)) = it_input.next() {
	    if ch.is_alphabetic() {
		attribute.key.push(ch);
	    } else if ch == ' ' {
		element.attributes.push(attribute);
		return Ok(AttributeEnding::Unfinished(&trimmed_input[i+1..]));
	    } else if ch == '=' {
		break;
	    } else if ch == '/' {
		if let Some((i, ch)) = it_input.next() {
		    if ch == '>' {
			element.attributes.push(attribute);
			return Ok(AttributeEnding::SelfClosing(&trimmed_input[i+1..]));
		    } else {
			return Err(String::from("Missing > in self-closing tag"));
		    }
		} else {
		    return Err(String::from("Could not parse /"));
		}
	    } else if ch == '>' {
		element.attributes.push(attribute);
		return Ok(AttributeEnding::RequiresClosing(&trimmed_input[i+1..]));
	    } else {
		return Err(String::from("Error parsing symbol: ") + &ch.to_string());
	    }
	}

	let mut value = String::new();

	while let Some((i, ch)) = it_input.next() {
	    if ch.is_alphanumeric() {
		value.push(ch);
	    } else if ch == ' ' {
		element.attributes.push(attribute);
		return Ok(AttributeEnding::Unfinished(&trimmed_input[i+1..]));
	    } else if ch == '/' {
		if let Some((i, ch)) = it_input.next() {
		    if ch == '>' {
			element.attributes.push(attribute);
			return Ok(AttributeEnding::SelfClosing(&trimmed_input[i+1..]));
		    } else {
			return Err(String::from("Missing > in self-closing tag"));
		    }
		} else {
		    return Err(String::from("Could not parse /"));
		}
	    } else if ch == '>' {
		element.attributes.push(attribute);
		return Ok(AttributeEnding::RequiresClosing(&trimmed_input[i+1..]));
	    } else {
		return Err(String::from("Error parsing symbol: ") + &ch.to_string());
	    }
	}

	return Ok(AttributeEnding::None);
    }
}

#[derive(Debug, PartialEq)]
struct Element {
    id :usize,
    name: String,
    attributes: Vec<Attribute>,
    parent_id: Option<Id>,
    children: VecDeque<Id>,
    children_stack :Vec<Id>,
}



impl<'a> Element {
    fn new(elements_table :&mut Vec<Element>, parent_id :Option<Id>, input :&'a str) -> Result<(Option<Element>, &'a str), String> {

	let delimiters :HashSet<char> = [' ', '>'].into();

	let mut element = Element {
	    id: elements_table.len(),
	    name: String::new(),
	    attributes: Vec::new(),
	    parent_id,
	    children: VecDeque::new(),
	    children_stack: Vec::new(),
	};

	let mut trimmed_input = input.trim_start();

	let mut it_input = trimmed_input.char_indices();

	let mut is_closing = false;

	if let Some((_, ch)) = it_input.next() {
	    if ch != '<' {
		return Err(String::from("Expected <, found: ") + &ch.to_string());
	    }
	} else {
	    return Err(String::from("Expected <, found nothing"));
	}

	if let Some((_, ch)) = it_input.next() {
	    if ch == '/' {
		is_closing = true;
	    } else {
		element.name.push(ch);
	    }
	} else {
	    return Err(String::from("Expected element name, found nothing"));
	}

	let mut next_idx :usize = 0;

	let mut covered_all_input = true;

	while let Some((i, ch)) = it_input.next() {
	    next_idx = i;
	    if ch.is_alphabetic() {
		element.name.push(ch);
	    } else if delimiters.contains(&ch) {
		covered_all_input = false;
		break;
	    } else {
		return Err(String::from("Wrong symbol: ") + &ch.to_string());
	    }
	}

	trimmed_input = if covered_all_input {
	    ""
	} else {
	    &trimmed_input[next_idx..]
	};

	let is_closed :bool;

	loop {
	    match Attribute::parse(&mut element, trimmed_input) {
		Ok(opt) => {
		    match opt {
			AttributeEnding::Unfinished(rest_of_input) => {
			    trimmed_input = rest_of_input;
			    continue;
			},

			AttributeEnding::SelfClosing(rest_of_input) => {
			    trimmed_input = rest_of_input;
			    is_closed = true;
			    break;
			},

			AttributeEnding::RequiresClosing(rest_of_input) => {
			    trimmed_input = rest_of_input;
			    is_closed = false;
			    break;
			},

			AttributeEnding::None => {
			    trimmed_input = "";
			    is_closed = true;
			    break;
			}
		    }
		},
		Err(e) => return Err(e),
	    };
	}

	if let Some(parent_id) = element.parent_id {
	    let parent = &mut elements_table[parent_id];

	    if is_closed {
		return Ok(( Some(element), trimmed_input));
	    }
	    
	    if is_closing {
		if let Some(prev_element_id) =
		    parent.children_stack.pop()
		{
		    let prev_element = &elements_table[prev_element_id];
		    
		    if prev_element.name != element.name {
			return Err(
			    String::from("Expected: ")
				+ &prev_element.name
				+ &String::from(", found: ")
				+ &element.name
			);
		    } else {
			return Ok(( Some(element), trimmed_input ));
		    }
		}

		return Err(String::from("Could not match closing tag: ") + &element.name);
	    }


	    let id :Id = element.id;

	    parent.children_stack.push(id);
	    element.children_stack.push(id);
	    elements_table.push(element);
	    return Element::new(elements_table, Some(id), trimmed_input);
	}

	element.id = 0;

	if is_closed {
	    elements_table.push(element);
	    return Ok(( None, trimmed_input));
	}

	if is_closing {
	    return Err(String::from("Could not match closing tag: ") + &element.name);
	}

	element.children_stack.push(element.id);
	elements_table.push(element);

	loop {
	    match Element::new(elements_table, Some(0), trimmed_input) {
		Ok((opt, input)) => {
		    let element_ptr = &mut elements_table[0];

		    if let Some(res_element) = opt {
			if res_element.name != element_ptr.name {
			    element_ptr.children.push_back(res_element.id);
			    elements_table.push(res_element);
			} else {
			    return Ok((None, input));
			}
		    }
		    
		    trimmed_input = input;
		},
		Err(e) => return Err(e)
	    };
	}

    }
}



#[derive(Debug, PartialEq)]
struct ElementTree {
    root_id: Id,
    elements_table: Vec<Element>
}


impl fmt::Display for ElementTree {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
	let mut cursor = &self.elements_table[self.root_id];
	let mut element_stack :VecDeque<Id> = VecDeque::new();

	cursor
	    .children
	    .iter()
	    .for_each(|element| {
		element_stack.push_back(*element);
	    });

	while let Some(element) = element_stack.pop_front() {
	    cursor = &self.elements_table[element];
	    cursor
		.children
		.iter()
		.for_each(|child| {
		    element_stack.push_back(*child);
		});
	    
	    if let Err(e) = write!(f, "-> {}\n", cursor.name) {
		return Err(e);
	    }
	};

	return Ok(());
    }
}


impl ElementTree {
    fn new(input :&str) -> Result<ElementTree, String> {
	let mut element_tree = ElementTree {
	    root_id: 0,
	    elements_table: Vec::new()
	};
	    
	match Element::new(&mut element_tree.elements_table, None, input) {
	    Ok((_, input)) => {
		if input != "" {
		    return Err(String::from("Could not parse rest of input: ") + input);
		}

		return Ok(element_tree);
	    },

	    Err(e) => {
		return Err(e);
	    }
	};
    }
}

mod tests;
