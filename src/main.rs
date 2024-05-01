use std::{env, fs};
use xml_interpreter::ElementTree;


fn main() {
    let mut args :Vec<String> = env::args().collect();
    let mut iter_args = args.iter_mut();
    
    iter_args.next();
    
    while let Some(arg) = iter_args.next() {
	if arg == "-h" || arg == "--help" {
	    println!("Help menu!");
	    break;
	}
	
	match fs::read_to_string(&arg) {
	    Ok(contents) => {
		match ElementTree::parse(&contents) {
		    Ok(result) => {
			println!("--- Contents of {} ---\n\n{}", arg, result.to_string());
		    },

		    Err(e) => panic!("Error: {}", e)
		};
	    },
	    
	    Err(e) => panic!("{:#?}", e)
	}
    }
}
