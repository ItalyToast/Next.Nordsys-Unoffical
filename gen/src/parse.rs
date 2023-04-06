use std::collections::HashMap;

use regex::{Regex, RegexBuilder, Captures};

lazy_static::lazy_static!{
    static ref CLASS_REGEX : Regex = RegexBuilder::new("Ext\\.define\\(\"MEM\\.model\\.([a-zA-Z]+)\".*?fields(.*?)],?\\s*},\\s*}\\);")
        .dot_matches_new_line(true)
        .build().unwrap();
    
    static ref FIELDS_REGEX : Regex = RegexBuilder::new("\\{([ \r\n]*)name: \"(\\w*)\",([ \r\n]*) type: \"(\\w*)\".*?\\}")
        .dot_matches_new_line(true)
        .build().unwrap();

    static ref PROP_REGEX : Regex = Regex::new("(\\w+?): ([^,\\}]+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct Class {
    pub class_name : String,
    pub class : String,
    pub fields : Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub text : String,
    pub name : String,
    pub datatype : String,
    pub always_present : bool,
    pub default_value : Option<String>,
    pub nullable : bool,
    pub persist : bool,
}

impl Field {
    pub fn new(name : &str, datatype : &str) -> Field {
        Field {
            text : String::new(),
            name : name.to_string(),
            datatype : datatype.to_string(),
            default_value : None,
            nullable : false,
            persist : true,
            always_present : false,
        }
    }

    pub fn base(name : &str, datatype : &str) -> Field {
        Field {
            always_present : true,
            ..Field::new(name, datatype)
        }
    }
}

pub fn parse_next_js() -> Vec<Class> {
    let data = std::fs::read_to_string("next.js").unwrap();

    let mut res :Vec<Class>= CLASS_REGEX.captures_iter(&data)
        .map(parse_class)
        .filter(|c| c.class_name != "Base")
        .collect();

    res.sort_by_key(|e| e.class_name.clone());

    res
}

fn parse_class(class_cap :Captures) -> Class {
    let class = class_cap.get(0).unwrap().as_str().to_string();
    let class_name = class_cap.get(1).unwrap().as_str().to_string();

    println!("class {}", &class);
    let base = vec![
        Field::base("Id", "int"),
        Field::new("Created", "date"),
        Field::new("CreatedId", "string"),
        Field::new("CreatedName", "string"),
        Field::new("Changed", "date"),
        Field::new("ChangedId", "string"),
        Field::new("Disabled", "boolean"),
        Field::new("isField", "boolean"),
        Field::new("isTmpRec", "boolean"),
    ];

    let mut fields:  HashMap<String, Field>= HashMap::new();
    for f in base {
        fields.insert(f.name.to_string(), f);
    }

    for field_cap in FIELDS_REGEX.captures_iter(&class)  {
        let f = parse_field(field_cap.get(0).unwrap().as_str());
        fields.insert(f.name.clone(), f);
    }
    
    let mut res = Class { 
        class_name, 
        class, 
        fields: fields.values().cloned().collect(),
    };

    res.fields.sort_by_key(|f| f.name.clone());

    res
}

fn parse_field(text :&str) -> Field {
    let mut props : HashMap<String, String> = HashMap::new();

    for prop in PROP_REGEX.captures_iter(text) {
        let key = prop.get(1).unwrap().as_str().to_string();
        let val = prop.get(2).unwrap().as_str();
        props.insert(key, val.trim().trim_matches('\"').to_string());
    }

    let name = props.get("name").unwrap();
    let datatype = props.get("type").unwrap();
    let default_value = props.get("defaultValue").map(|p| translate(p));

    Field { 
        text: text.to_string(), 
        name: name.to_string(), 
        datatype: datatype.to_string(), 
        default_value: default_value, 
        nullable: props.contains_key("useNull"), 
        persist: !props.contains_key("persist"),
        always_present: false,
    }
}

fn translate(text : &str) -> String {
    let res = match text {
        "!0" => "true",
        "!1" => "false",
        _ => text,
    };
    res.to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_contains() {
        let classes = parse_next_js();
        assert!(classes.iter().any(|p| p.class_name == "ProfessionItem"));
        assert!(classes.iter().any(|p| p.class_name == "WorkOrderRow"));
    }
}