#[cfg(test)]
use super::*;


#[test]
fn test_parse() {
    let mut element = Element::new_root();

    let mut element_solution = Element::new_root();
    element_solution.children = vec!(
        Element {
            name: "hello".to_string(),
            attributes: Vec::new(),
            children: Vec::new()
        }
        );

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


#[test]
fn test_extract_attribute() {
    assert_eq!(
        extract_attributes("hello=world text=5>"),
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

