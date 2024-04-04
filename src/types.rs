pub type Attribute = (String, String);

#[derive(PartialEq, Debug)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new() -> Element {
        Element {
            name: String::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn new_root() -> Element {
        let mut root = Element::new();
        root.name = "root".to_string();

        return root;
    }
}


