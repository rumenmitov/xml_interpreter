use std::collections::{VecDeque, HashSet};
use std::fmt;






type Attribute = (String, String);

struct Element {
    id :usize,
    name: String,
    attributes: Vec<Attribute>,
    parent_id: Option<usize>,
    children: VecDeque<usize>,
    children_stack :Vec<usize>,
}



impl<'a> Element {
    fn new(elements_table :&mut Vec<Element>, parent_id :Option<usize>, input :&'a str, delimiters :&HashSet<char>) -> Result<(Option<Element>, &'a str), String> {

	let mut element = Element {
	    id: elements_table.len(),
	    name: String::new(),
	    attributes: Vec::new(),
	    parent_id,
	    children: VecDeque::new(),
	    children_stack: Vec::new()
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

	if let Some(parent_id) = element.parent_id {
	    let parent = &mut elements_table[parent_id];
	    
	    if is_closing {			
		if let Some(prev_element_id) =
		    parent.children_stack.pop()
		{
		    let prev_element = &elements_table[prev_element_id];
		    
		    if prev_element.name != element.name {
			return Err(
			    String::from("Expected: ")
				+ &prev_element.name
				+ &String::from(" , found: ")
				+ &element.name
			);
		    } else {
			return Ok(( Some(element), trimmed_input ));
		    }
		}

		return Err(String::from("Could not match closing tag: ") + &element.name);
	    }


	    parent.children_stack.push(element.id);
	    return Element::new(elements_table, Some(element.id), trimmed_input, delimiters);
	}

	element.id = 0;

	if is_closing {
	    return Err(String::from("Could not match closing tag: ") + &element.name);
	}

	element.children_stack.push(element.id);
	elements_table.push(element);

	loop {
	    match Element::new(elements_table, Some(0), trimmed_input, delimiters) {
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



struct ElementTree {
    root_id: usize,
    elements_table: Vec<Element>
}


impl fmt::Display for ElementTree {
    fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
	let mut cursor = &self.elements_table[self.root_id];
	let mut element_stack :VecDeque<usize> = VecDeque::new();

	
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
	    
	let element_name_delimiters :HashSet<char> = vec![' '].into_iter().collect();

	match Element::new(&mut element_tree.elements_table, None, input, &element_name_delimiters) {
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
