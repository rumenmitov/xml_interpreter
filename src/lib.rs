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

fn extract_name(input: &str) -> Result<Box<(String, String)>, &str> {
    let mut result = String::new();
    let mut it = input.char_indices();

    while let Some((i, ch)) = it.next() {
        if ch.is_alphanumeric() {
            result.push(ch);
        } else if ch == ' ' {
            extract_attributes(&input[i..]);
        }
    }

    let mut rest_of_input = String::new();

    if let Some((i, _)) = it.next() {
        rest_of_input = input[i..].to_string();
    }

    Ok(Box::from((result, rest_of_input)))
}

fn extract_attributes(input :&str) -> Result<Box<Vec<Attribute>>, &str> {
    let mut attribute = (String::new(), String::new());
    let mut attributes = Vec::new();
    let mut it = input.chars();

    while let Some(ch) = it.next() {
        if ch.is_alphabetic() {
            attribute.0.push(ch);
        } else if ch == '=' {
            break;
        } else {
            return Err("Missing '=' in attribute assignment.")
        }
    }

    while let Some(ch) = it.next() {
        if ch.is_alphanumeric() {
            attribute.1.push(ch);
        } else if ch == ' '{
            attributes.push((attribute.0, attribute.1));
            attribute.0 = String::new();
            attribute.1 = String::new();
        } else if ch == '/' && it.next() == Some('>') {
            break;
        } else {
            return Err("Attribute value set incorrectly!");
        }
    }

    return Ok(Box::new(attributes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_attribute() {
        assert_eq!(
            extract_attribute("hello"),
            Ok(Box::from((String::from("hello"), String::from(""))))
        );

        assert_eq!(
            extract_attribute("!hello"),
            Ok(Box::from((String::from(""), String::from("hello"))))
        );
    }

    #[test]
    fn test_parse() {
        let mut element = Element::new();

        let mut element_solution = Element {
            attribute: "".to_string(),
            children: vec![Element {
                attribute: "hello".to_string(),
                children: Vec::new(),
            }],
        };

        assert_eq!(
            parse("<hello>".to_string(), element).unwrap(),
            (element_solution, "".to_string())
        );

        element = Element::new();

        element_solution = Element {
            attribute: "".to_string(),
            children: vec![Element {
                attribute: "hello".to_string(),
                children: vec!(Element {
                    attribute: "nested".to_string(),
                    children: Vec::new()
                }),
            }],
        };

        assert_eq!(
            parse("<hello><nested>".to_string(), element).unwrap(),
            (element_solution, "".to_string())
        );
    }
}
