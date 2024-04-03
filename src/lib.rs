type Attribute = (String, String);
type ParseResult<'a> = Result<(Element, String), String>;

#[derive(PartialEq, Debug)]
struct Element {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
}

impl Element {
    fn new() -> Element {
        Element {
            name: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    fn new_root() -> Element {
        let mut root = Element::new();
        root.name = "root".to_string();
        return root;
    }
}



fn parse<'a>(input: &str, mut parent: Element) -> ParseResult<'a> {
    println!("INPUT: {:?}", input);
    if input.is_empty() {
        return Ok((parent, String::new()));
    }

    if !input.starts_with("<") && !input.starts_with("</") {
        return Err("Error while parsing: ".to_string() + input);
    }

    let name = extract_name(&input[1..]).unwrap();
    if name.0 == '/'.to_string() + &parent.name {
        if !name.1.starts_with(">") {
            return Err("Closing tag incorrect: ".to_string() + &name.1);
        }
        return Ok((parent, name.1[(1 + name.0.len())..].to_string()));
    }

    if name.1.starts_with("/>") {
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

    return Ok((parent, rest_of_input));
}

fn extract_name(input: &str) -> Result<Box<(String, String)>, String> {
    let mut result = String::new();
    let mut it = input.char_indices();

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

fn extract_attributes(input :&str) -> Result<Box<(Vec<Attribute>, String)>, String> {
    let mut attribute = (String::new(), String::new());
    let mut attributes = Vec::new();
    let mut it = input.char_indices();

    if let Some((i, ch)) = it.next() {
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

    while let Some((_, ch)) = it.next() {
        if ch.is_alphabetic() {
            attribute.0.push(ch);
        } else if ch == '=' {
            break;
        } else {
            return Err("Error parsing attribute: ".to_string() + input)
        }
    }

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

    let mut rest_of_input = String::new();

    if let Some((i, _)) = it.next() {
        rest_of_input += &input[i..];
    }

    let rest_of_attributes = *extract_attributes(&rest_of_input).unwrap();
    rest_of_attributes.0.into_iter().for_each(|attr| {
        attributes.push(attr);
    });

    return Ok(Box::new((attributes, rest_of_attributes.1)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_attribute() {
        assert_eq!(
            extract_attributes("hello=world text=5"),
            Ok(Box::from((
                        vec!(
                            ("hello".to_string(), "world".to_string()),
                            ("text".to_string(), "5".to_string())
                             ), 
                        String::from("")
                        )))
        );

        assert_eq!(
            extract_attributes("!hello"),
            Err("Error parsing attribute: !hello".to_string())
        );
    }

    #[test]
    fn test_extract_name() {
        assert_eq!(
            extract_name("hello>"),
            Ok(
                Box::from(("hello".to_string(), ">".to_string())))
            );

        assert_eq!(
            extract_name("hello/>"),
            Ok(
                Box::from(("hello".to_string(), "/>".to_string())))
            );

        assert_eq!(
            extract_name("5hello>"),
            Err("Invalid character when extracting name: 5".to_string())
            );
    }

    #[test]
    fn test_parse() {
        let mut element = Element::new_root();

        let mut element_solution = Element::new_root();
        element_solution.children = vec!(Element::new());

        assert_eq!(
            parse("<hello></hello>", element).unwrap(),
            (element_solution, "".to_string())
        );

        element = Element::new_root();

        element_solution = Element::new_root();
        element_solution.children = vec!(
            Element {
                name: "hello".to_string(),
                attributes: vec!(
                    ("size".to_string(), "10".to_string()),
                    ("text".to_string(), "5".to_string())
                    ),
                    children: vec!(
                        Element {
                            name: "nested".to_string(),
                            attributes: vec!(),
                            children: vec!(),
                        }
                        ),
            }
        );

        assert_eq!(
            parse("<hello size=10 text=5><nested></nested></hello>", element).unwrap(),
            (element_solution, "".to_string())
        );
    }
}
