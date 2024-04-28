#[cfg(test)]
use super::*;

#[test]
fn test_element_tree() {

    let et = ElementTree::new("<root><text width=5><img /></text></root>").unwrap();
    assert_eq!("root", et.elements_table[0].name);
    assert_eq!("text", et.elements_table[1].name);
    assert_eq!("img", et.elements_table[2].name);
}
