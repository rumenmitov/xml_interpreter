use std::collections::{VecDeque, HashSet};
use std::fmt;



type Id = usize;

#[derive(Debug, PartialEq, Clone)]
struct Attribute {
    key: String,
    value: Option<String>,
}

#[derive(Debug, PartialEq)]
enum AttributeEnding<'a> {
    Unfinished((Attribute, &'a str)),
    SelfClosing((Attribute, &'a str)),
    RequiresClosing((Attribute, &'a str)),
    None,
}

impl<'a> Attribute {    
    fn is_empty(&self) -> bool {
	self.key.is_empty() && self.value == None
    }
    
    fn parse(input :&'a str) -> Result<AttributeEnding<'a>, String> {
	let mut attribute = Attribute {
	    key: String::new(),
	    value: None
	};

	let trimmed_input = input.trim_start();

	let mut iter_input = trimmed_input.char_indices();

	while let Some((i, ch)) = iter_input.next() {
	    if ch.is_alphabetic() {
		attribute.key.push(ch);
	    } else if ch == ' ' {
		return Ok(AttributeEnding::Unfinished((attribute, &trimmed_input[i+1..])));
	    } else if ch == '=' {
		break;
	    } else if ch == '/' {
		if let Some((i, ch)) = iter_input.next() {
		    if ch == '>' {
			return Ok(AttributeEnding::SelfClosing((attribute, &trimmed_input[i+1..])));
		    } else {
			return Err(String::from("Missing > in self-closing tag"));
		    }
		} else {
		    return Err(String::from("Could not parse /"));
		}
	    } else if ch == '>' {
		return Ok(AttributeEnding::RequiresClosing((attribute, &trimmed_input[i+1..])));
	    } else {
		return Err(String::from("Error parsing symbol: ") + &ch.to_string());
	    }
	}

	attribute.value = Some(String::new());

	while let Some((i, ch)) = iter_input.next() {
	    if ch.is_alphanumeric() {
		if let Some(val) = attribute.value {
		    attribute.value = Some(val + &ch.to_string());
		}
	    } else if ch == ' ' {
		return Ok(AttributeEnding::Unfinished((attribute, &trimmed_input[i+1..])));
	    } else if ch == '/' {
		if let Some((i, ch)) = iter_input.next() {
		    if ch == '>' {
			return Ok(AttributeEnding::SelfClosing((attribute, &trimmed_input[i+1..])));
		    } else {
			return Err(String::from("Missing > in self-closing tag"));
		    }
		} else {
		    return Err(String::from("Could not parse /"));
		}
	    } else if ch == '>' {
		return Ok(AttributeEnding::RequiresClosing((attribute, &trimmed_input[i+1..])));
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
    children: Vec<Id>,
}


#[derive(Debug, PartialEq)]
enum ElementState<'a> {
    Opening((Element, &'a str)),
    Closing((Element, &'a str)),
    SelfClosing((Element, &'a str)),
    None,
}


impl<'a> Element {
    fn new() -> Element {
	Element {
	    id: 0,
	    name: String::new(),
	    attributes: Vec::new(),
	    parent_id: None,
	    children: Vec::new()
	}
    }
    
    fn parse(id :Id, original_input :&'a str) -> Result<ElementState<'a>, String> {
	let mut element = Element::new();
	element.id = id;

	let mut element_state  = ElementState::None;
	
	let delimiters :HashSet<char> = [' ', '>'].into();

	let mut input = original_input.trim_start();

	let mut iter_input = input.char_indices();

	if let Some((_, ch)) = iter_input.next() {
 	    if ch != '<' {
 		return Err(String::from("Expected <, found: ") + &ch.to_string());
 	    }
 	} else {
 	    return Err(String::from("Expected <, found nothing"));
 	}

 	if let Some((_, ch)) = iter_input.next() {
 	    if ch == '/' {
 		element_state = ElementState::Closing((Element::new(), ""));
 	    } else {
 		element.name.push(ch);
 	    }
 	} else {
 	    return Err(String::from("Expected element name, found nothing"));
	}

	let mut covered_all_input = true;

	while let Some((i, ch)) = iter_input.next() {
 	    if ch.is_alphabetic() {
 		element.name.push(ch);
 	    } else if delimiters.contains(&ch) {
		covered_all_input = false;
		input = &input[i..];
 		break;
 	    } else {
 		return Err(String::from("Wrong symbol: ") + &ch.to_string());
 	    }
	}

	input = if covered_all_input {
	    ""
	} else {
	    input
	};

	if let ElementState::Closing(_) = element_state {
	    return Ok(ElementState::Closing((element, &input[1..])));
	}

	loop {
	    match Attribute::parse(input) {
		Ok(opt) => {
		    match opt {
			AttributeEnding::Unfinished((attribute, rest_of_input)) => {
			    if !attribute.is_empty() {
				element.attributes.push(attribute);
			    }
			    
			    input = rest_of_input;
			    continue;
			},

			AttributeEnding::SelfClosing((attribute, rest_of_input)) => {
			    if !attribute.is_empty() {
				element.attributes.push(attribute);
			    }
			    
			    return Ok(ElementState::SelfClosing((element, rest_of_input)));
			},

			AttributeEnding::RequiresClosing((attribute, rest_of_input)) => {
			    if !attribute.is_empty() {
				element.attributes.push(attribute);
			    }
			    
			    return Ok(ElementState::Opening((element, rest_of_input)));
			},

			AttributeEnding::None => {
			    return Ok(ElementState::None);
			}
		    }
		},
		Err(e) => return Err(e),
	    };
 	}
    }
}


#[derive(Debug, PartialEq)]
struct ElementTree {
    root_id: Id,
    elements: Vec<Element>
}


impl fmt::Display for ElementTree {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
	let mut cursor = &self.elements[self.root_id];
	let mut element_stack :VecDeque<Id> = VecDeque::new();

	cursor
	    .children
	    .iter()
	    .for_each(|element| {
		element_stack.push_back(*element);
	    });

	while let Some(element) = element_stack.pop_front() {
	    cursor = &self.elements[element];
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
    fn new(original_input :&str) -> Result<ElementTree, String> {
	let mut element_tree = ElementTree {
	    root_id: 0,
	    elements: Vec::new(),
	};

	let mut unclosed_elements_stack :Vec<Id> = Vec::new();
	let mut parent_id :Option<Id> = None;
	let mut current_id :Id = 0;

	let mut input = original_input;

	while input != "" {
	    match Element::parse(current_id, input) {
		Ok(element_state) => {
		    match element_state {
			ElementState::Opening((mut element, rest_of_input)) => {
			    element.parent_id = parent_id;
			    
			    element_tree.elements.push(element);
			    input = rest_of_input;

			    if let Some(id) = parent_id {
				unclosed_elements_stack.push(id);

				if let Some(parent) = element_tree.elements.iter_mut().nth(id) {
				    parent.children.push(current_id);
				}
			    }

			    parent_id = Some(current_id);
			},
			
			ElementState::SelfClosing((mut element, rest_of_input)) => {
			    element.parent_id = parent_id;
			    
			    element_tree.elements.push(element);
			    input = rest_of_input;

			    if let Some(id) = parent_id {
				unclosed_elements_stack.push(id);

				if let Some(parent) = element_tree.elements.iter_mut().nth(id) {
				    parent.children.push(current_id);
				}
			    }
			},
			
			ElementState::Closing((element, rest_of_input)) => {
			    if element_tree.elements.is_empty() {
				return Err(String::from("Closing tag missing opening tag! Element: ")
					   + &element.name);
			    }

			    if let Some(prev_element_id) = unclosed_elements_stack.pop() {
				if let Some(prev_element) =
				    element_tree.elements.iter().nth(prev_element_id)
				{
				    if prev_element.name != element.name {
					return Err(
					    String::from("Opening and closing tag mismatch! Expected: ")
						+ &prev_element.name
						+ ", found: "
						+ &element.name
					);
				    } else {
					input = rest_of_input;
					parent_id = prev_element.parent_id;
					continue;
				    }
				}
			    }
			},
			    
			ElementState::None => {
			    break;
			}
		    }
		},
		Err(e) => {
		    return Err(e)
		}
	    }

	    current_id += 1;
	}
	
	return Ok(element_tree);
    }
}

mod tests;
