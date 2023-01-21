use crate::parse::{self, Class, Field};

pub fn gen_kotlin_from_next_js() {
    let classes = parse::parse_next_js();

    let mut result = String::new();

    result.push_str("/* ");
    result.push_str(&read_license());
    result.push_str(" */\r\n");
    result.push_str("\r\n");
    result.push_str("// This file is generated by the project found in gen/ \r\n");
    result.push_str("\r\n");
    
    result.push_str("// Available tables:\r\n");
    for class in &classes {
        result.push_str(&format!("// -{}Store\r\n", &class.class_name));
    }
    result.push_str("\r\n");

    for class in &classes {
        result.push_str(&gen_class(class));
    }

    std::fs::write("next.js.kt", result).unwrap();
}

fn gen_class(class : &Class) -> String {
    let mut class_res = String::new();

    class_res.push_str("/*\r\n");
    class_res.push_str(&class.class);
    class_res.push_str("*/\r\n");

    class_res.push_str("@Suppress(\"unused\")\r\n");
    class_res.push_str(&format!("class {}Store (\r\n", &class.class_name));

    for field in &class.fields {
        class_res.push_str(&gen_field(&field));
    }

    class_res.push_str(")\r\n\r\n");

    class_res
}

fn gen_field(field : &Field) -> String {
    let mut text = String::new();
    match field.nullable {
        true => {
            text.push_str(&format!("    val {} : String?, //{}\r\n", field.name, field.datatype));
        },
        false => {
            text.push_str(&format!("    val {} : String, //{}\r\n", field.name, field.datatype));
        },
    };

    text
}

fn read_license() -> String {
    let content = std::fs::read("LICENSE").unwrap();
    String::from_utf8(content).unwrap()
}
