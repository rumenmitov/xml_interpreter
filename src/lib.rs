#[derive(PartialEq, Debug)]
struct Element {
    name: String,
    attribute: Attribute,
    children: Vec<Element>,
}

impl Element {
    fn new() -> Element {
        Element {
            name: String::new(),
            attribute: (String::new(), String::new()),
            children: Vec::new(),
        }
    }
}

type Attribute = (String, String);

type ParseResult<'a> = Result<(Element, String), &'a str>;

fn parse<'a>(input: &str, mut parent: Element) -> ParseResult<'a> {
    let mut element = Element::new();
    let mut rest_of_input = String::new();

    if input.starts_with("<") {
        let extraction_result = extract_name(&input[1..]).unwrap();
        element.name = extraction_result.0;
        rest_of_input = extraction_result.1;
    }

    if element.name == parent.name {
        return Ok((parent, rest_of_input));
    }

    let final_result = parse(&rest_of_input, element)?;
    element = final_result.0;
    rest_of_input = final_result.1.to_string();

    if element != Element::new() {
        parent.children.push(element);
    }

    Ok((parent, rest_of_input))
}

fn extract_name(input: &str) -> Result<Box<(String, String)>, String> {
    let mut result = String::new();
    let mut it = input.char_indices();

    while let Some((_, ch)) = it.next() {
        if ch.is_alphabetic() {
            result.push(ch);
        } else if ch == ' ' || ch == '>' {
            break;
        } else {
            let mut err = String::from("Invalid character when extracting name: ");
            err.push(ch);
            return Err(err);
        }
    };

    let mut rest_of_input = String::new();

    if let Some((i, _)) = it.next() {
        rest_of_input = input[i..].to_string();
    }

    Ok(Box::from((result, rest_of_input)))
}

fn extract_attributes(input :&str) -> Result<Box<(Vec<Attribute>, String)>, &str> {
    let mut attribute = (String::new(), String::new());
    let mut attributes = Vec::new();
    let mut it = input.char_indices();

    if let Some((_, ch)) = it.next() {
        if ch == '/' || ch == ' ' || ch == '>' {
            return Ok(Box::from((vec!(), String::from(input))));
        }

        if ch.is_alphabetic() {
            attribute.0.push(ch);
        } else {
            return Err("Error parsing attribute.")
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
            return Err("Error parsing attribute.")
        }
    }

    while let Some((_, ch)) = it.next() {
        if ch.is_alphanumeric() {
            attribute.1.push(ch);
        } else if ch == ' ' || ch == '/' || ch == '>' {
            break;
        } else {
            return Err("Attribute value set incorrectly!");
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
                            (String::from("hello"), String::from("world")),
                            (String::from("text"), String::from("5"))
                             ), 
                        String::from("")
                        )))
        );

        assert_eq!(
            extract_attributes("!hello"),
            Err("Error parsing attribute.")
        );
    }

    #[test]
    fn test_extract_name() {
        assert_eq!(
            extract_name("hello>"),
            Ok(Box::from(
                    (String::from("hello"), String::from(""))
                    ))
            );

        assert_eq!(
            extract_name("5hello>"),
            Err(String::from("Invalid character when extracting name: 5"))
            );
    }

    #[test]
    #[ignore]
    fn test_parse() {
        // let mut element = Element::new();

        // let mut element_solution = Element {
        //     attribute: "".to_string(),
        //     children: vec![Element {
        //         attribute: "hello".to_string(),
        //         children: Vec::new(),
        //     }],
        // };

        // assert_eq!(
        //     parse("<hello>".to_string(), element).unwrap(),
        //     (element_solution, "".to_string())
        // );

        // element = Element::new();

        // element_solution = Element {
        //     attribute: "".to_string(),
        //     children: vec![Element {
        //         attribute: "hello".to_string(),
        //         children: vec!(Element {
        //             attribute: "nested".to_string(),
        //             children: Vec::new()
        //         }),
        //     }],
        // };

        // assert_eq!(
        //     parse("<hello><nested>".to_string(), element).unwrap(),
        //     (element_solution, "".to_string())
        // );
    }
}
