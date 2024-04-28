#[cfg(test)]
use super::*;

#[test]
fn test_element_tree() {

    let et = ElementTree::new("<root </root").unwrap();
    assert_eq!("root", et.elements_table[0].name);
}
