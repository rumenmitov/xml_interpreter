//! # XML Interpreter
//!
//! This crate parses basic XML syntax.
//! XML syntax supported: element name, closing tags, nesting, attributes
//! (without the "").
//!
//! ## Example Usage
//!
//! ```xml
//! <form>
//!     <input width=10 height=5 />
//! </form>
//! ```
//!
//! ## Credits
//!
//! This project was inspired by https://bodil.lol/parser-combinators/.
//! However, I found the implementation convoluted so I tried writing my own
//! version.


mod types;
pub use crate::types::*;


/// Parses a string (with no `\n`) and returns the filled-in parent element 
/// (i.e. the root).
///
/// # Throws
/// String of the parsing error.
pub fn parse<'a>(input: &str, mut parent: Element) -> Result<(Element, String), String> {
    if input.is_empty() {
        return Ok((parent, String::new()));
    }

    if !input.starts_with("<") && !input.starts_with("</") {
        return Err("Error while parsing: ".to_string() + input);
    }

    let name = extract_name(&input[1..]).unwrap();

    if let Some(prev_child) = parent.children.last() {
        // INFO: Handles the case of </parent>.
        if name.0 == prev_child.name {
            if !name.1.starts_with(">") {
                return Err("Closing tag incorrect: ".to_string() + &name.1);
            }
            return Ok((parent, name.1[1..].to_string()));
        }
    }

    if name.1.starts_with("/>") {
        // INFO: Handles the case of <img/>
        parent.children.push(
            Element {
                name: name.0,
                attributes: Vec::new(),
                children: Vec::new()
            }
            );

        return Ok((parent, name.1[2..].to_string()));
    }

    if name.1.starts_with(">") {
        // INFO: Removes closing tag from previos parse.
        parent.children.push(
            Element {
                name: name.0,
                attributes: Vec::new(),
                children: Vec::new()
            }
            );

        return parse(&name.1[1..], parent);
    }

    let attributes = extract_attributes(&name.1[1..]).unwrap();

    let mut element = Element::new();
    element.name = name.0;
    element.attributes = attributes.0;

    let (element, rest_of_input) = parse(&attributes.1, element).unwrap();
    parent.children.push(element);

    return parse(&rest_of_input, parent);
}

/// Returns the a tuple of the name of an element and the rest of the string.
///
/// # Throws
/// String of the parsing error.
fn extract_name(input: &str) -> Result<Box<(String, String)>, String> {
    let mut result = String::new();
    let mut it = input.char_indices();

    // INFO: Checks for case </element>.
    if let Some((_, ch)) = it.next() {
        if ch != '/' && ch.is_alphabetic() {
            result.push(ch);
        } else if ch != '/' && !ch.is_alphabetic() {
            return Err("Invalid character when extracting name: ".to_string() + &ch.to_string())
        }
    }

    while let Some((i, ch)) = it.next() {
        if ch.is_alphabetic() {
            result.push(ch);
        } else if ch == ' ' || ch == '>' || ch == '/' {
            return Ok(Box::from((result, input[i..].to_string())))
        } else {
            return Err("Invalid character when extracting name: ".to_string() + &ch.to_string());
        }
    };

    return Ok(Box::from((result, String::new())));

}

/// Returns the a tuple of all the attributes and the rest of the string.
///
/// # Throws
/// String of the parsing error.
fn extract_attributes(input :&str) -> Result<Box<(Vec<Attribute>, String)>, String> {
    let mut attribute = (String::new(), String::new());
    let mut attributes = Vec::new();
    let mut it = input.char_indices();

    if let Some((i, ch)) = it.next() {
        // INFO: Handles cases when attribute list is over and we should parse
        // the next element, otherwise adds character to the attribute list.
        if ch == '<' {
            return Ok(Box::from((vec!(), String::from(&input[i..]))));
        }

        if ch == '/' || ch == ' ' || ch == '>' {
            return Ok(Box::from((vec!(), String::from(&input[(i+1)..]))));
        }

        if ch.is_alphabetic() {
            attribute.0.push(ch);
        } else {
            return Err("Error parsing attribute: ".to_string() + input)
        }

    } else {
        return Ok(Box::from((vec!(), String::from(input))));
    }

    // INFO: Extracts attribute name.
    while let Some((_, ch)) = it.next() {
        if ch.is_alphabetic() {
            attribute.0.push(ch);
        } else if ch == '=' {
            break;
        } else {
            return Err("Error parsing attribute: ".to_string() + input)
        }
    }

    // INFO: Extracts attribute value.
    while let Some((_, ch)) = it.next() {
        if ch.is_alphanumeric() {
            attribute.1.push(ch);
        } else if ch == ' ' || ch == '/' || ch == '>' {
            break;
        } else {
            return Err("Attribute value set incorrectly: ".to_string() + input);
        }
    }

    attributes.push((attribute.0, attribute.1));

    if let Some((i, _)) = it.next() {
        let rest_of_attributes = *extract_attributes(&input[i..]).unwrap();
        rest_of_attributes.0.into_iter().for_each(|attr| {
            attributes.push(attr);
        });

        return Ok(Box::new((attributes, rest_of_attributes.1)));

    } else {
        return Ok(Box::new((attributes, String::new())));
    }
}

mod tests;
