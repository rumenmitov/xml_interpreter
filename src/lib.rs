#[derive(PartialEq, Debug)]
struct Element {
    attribute: String,
    children: Vec<Element>,
}

impl Element {
    fn new() -> Element {
        Element {
            attribute: String::new(),
            children: Vec::new(),
        }
    }
}

type ParseResult<'a> = Result<(Element, String), &'a str>;

fn parse<'a>(input: String, mut parent: Element) -> ParseResult<'a> {
    let mut element = Element::new();
    let mut rest_of_input = String::new();

    if input.starts_with("<") {
        let extraction_result = extract_attribute(&input[1..]).unwrap();
        element.attribute = extraction_result.0;
        rest_of_input = extraction_result.1;
    }

    if element.attribute == parent.attribute {
        return Ok((parent, rest_of_input));
    }

    let final_result = parse(rest_of_input, element)?;
    element = final_result.0;
    rest_of_input = final_result.1.to_string();

    if element != Element::new() {
        parent.children.push(element);
    }

    Ok((parent, rest_of_input))
}

fn extract_attribute(input: &str) -> Result<Box<(String, String)>, &str> {
    let mut result = String::new();
    let mut it = input.chars();

    while let Some(ch) = it.next() {
        if ch.is_alphanumeric() {
            result.push(ch);
        } else {
            break;
        }
    }

    let mut rest_of_input = String::new();

    while let Some(ch) = it.next() {
        rest_of_input += &ch.to_string();
    }

    Ok(Box::from((result, rest_of_input)))
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
