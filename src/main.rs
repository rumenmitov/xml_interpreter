use std::io;

use xml_interpreter::*;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin.");

    input = input
        .trim()
        .replace("\t", "");

    let root = Element::new_root();

    if let Ok((root, _)) = parse(&input, root) {
        print!("{}", root.print(0));
    }
}
