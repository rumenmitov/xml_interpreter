/// Contains the attribute name and attribute value.
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

    pub fn print(&self, distance_from_root :u8) -> String {
        let mut tab_offset = String::new();
        for _ in 1..distance_from_root {
            tab_offset.push(' ');
            tab_offset.push(' ');
        }

        let mut res = String::from(&self.name);

        for (name, val) in &self.attributes {
            res += ", ";
            res += name;
            res += "=";
            res += val;
        }

        if !self.children.is_empty() {
            tab_offset.push(' ');
            tab_offset.push(' ');

            res.push('\n');
            res += &tab_offset;
            res += "|\n";

            for child in &self.children {
                res += &tab_offset;
                res += "-->";

                res += &child.print(distance_from_root + 1);

                res.push('\n');
            }
        }
        
        return res;
    }
}
