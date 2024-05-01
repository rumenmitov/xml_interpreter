use std::{env, fs};
use xml_interpreter::ElementTree;


fn main() {
    for filename in env::args() {
	if filename == "-h" || filename == "--help" {
	    todo!();
	    continue;
	}
	
	match fs::read_to_string(&filename) {
	    Ok(contents) => {
		match ElementTree::parse(&contents) {
		    Ok(result) => {
			println!("--- Contents of {}: ---\n{}\n\n", filename, result);
		    },

		    Err(e) => panic!("Error: {}", e)
		};
	    },
	    
	    Err(e) => panic!("{:#?}", e)
	}
    }
}
