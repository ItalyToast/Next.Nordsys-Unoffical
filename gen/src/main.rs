use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::{RegexBuilder, Regex};

mod langpack;

lazy_static::lazy_static!{
    static ref PROP_REGEX: Regex = Regex::new("(\\w+?): ([^,\\}]+)").unwrap();
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    pub success : bool,
    pub rows : Option<Vec<T>>,
}

struct Field {
    text : String,
    name : String,
    datatype : String,
    always_present : bool,
    default_value : Option<String>,
    nullable : bool,
    persist : bool,
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

    pub fn format(&self) -> String {
        let mut text = String::new();
        if !self.always_present {
            text.push_str(&format!("    #[serde(default)]\r\n"));
        }
        match self.nullable {
            true => {
                text.push_str(&format!("    pub {} : Option<String>, //{}\r\n", self.name, self.datatype));
            },
            false => {
                text.push_str(&format!("    pub {} : String, //{}\r\n", self.name, self.datatype));
            },
        };

        text
    }
}

fn main() {
    gen_from_next_js();
    langpack::run_codegen();
}


pub fn gen_from_json_file(name : &str) {
    let content = std::fs::read("json.txt").unwrap();
    let json = String::from_utf8(content).unwrap();
    gen_from_json(name, &json);
}

fn gen_from_json(name : &str, json : &str) {
    match serde_json::from_str::<ApiResponse<Value>>(json) {
        Ok(json) => {
            match json.rows {
                Some(res) => {
                    if let Some(s) = res.first() {
                        let s = gen_struct(name, s);
                        let mut path = String::new();
                        path.push_str("gen/");
                        path.push_str(name);
                        path.push_str(".txt");
                        std::fs::write(path, s).unwrap();
                    }
                },
                None => println!("No rows returned"),
            }
        },
        Err(err) => {
            println!("[{}]{:?}", name, err);
        },
    }
}

fn gen_struct(name : &str, json : &Value) -> String {
    let mut code = String::new();
    code.push_str("#[allow(non_snake_case)]\r\n");
    code.push_str("#[derive(Serialize, Deserialize, Clone)]\r\n");
    code.push_str("pub struct ");
    code.push_str(name);
    code.push_str(" {\r\n");

    for (k, _v) in json.as_object().unwrap() {
        code.push_str("    pub ");
        code.push_str(k);
        code.push_str(" : String,\r\n");
    }

    code.push_str("}");
    code
}

fn gen_from_next_js() {
    /*
    Ext.define("MEM.model.AccountChart", {
        extend: MEM.model.Base,
        config: {
            fields: [
            { name: "AccountNo", type: "string", meType: "meShortText", max: 5 },
            {
                name: "Account",
                type: "string",
                meType: "meShortText",
                max: 5,
                convert: function (b, a) {
                return a.get("AccountNo") + " " + a.get("Description");
                },
            },
            { name: "Description", type: "string", meType: "meMediumText", max: 50 },
            { name: "Cost", type: "bool", meType: "meBool" },
            { name: "Work", type: "bool", meType: "meBool" },
            { name: "Material", type: "bool", meType: "meBool" },
            { name: "VatCode", type: "string", meType: "meShortText", max: 10 },
            {
                name: "ComplementAccount",
                type: "string",
                meType: "meShortText",
                max: 5,
            },
            { name: "VatDuty", type: "bool", meType: "meBool" },
            { name: "InvoiceOnImport", type: "bool", meType: "meBool" },
            { name: "RotShare", type: "int", meType: "meInt" },
            ],
        },
        });
        Ext.define("MEM.store.MyAccountChartStore", {
        extend: MEM.ux.data.MyBaseStore,
        config: {
            model: "MEM.model.AccountChart",
            storeId: "MyAccountChartStore",
            autoLoad: !1,
            autoSync: !1,
            remoteFilter: !1,
            sortOnLoad: !0,
            sorters: [{ property: "AccountNo", directon: "ASC" }],
            proxy: { type: "myproxy", url: Me.STORE_URL + "AccountChartStore" },
        },
        });
        */

    let class_regex = RegexBuilder::new("Ext\\.define\\(\"MEM\\.model\\.([a-zA-Z]+)\".*?fields(.*?)]")
        .dot_matches_new_line(true)
        .build().unwrap();
    let fields_regex = RegexBuilder::new("\\{([ \r\n]*)name: \"(\\w*)\",([ \r\n]*) type: \"(\\w*)\".*?\\}")
        .dot_matches_new_line(true)
        .build().unwrap();

    let data = std::fs::read_to_string("next.js").unwrap();

    let mut result = String::new();
    result.push_str("/* ");
    result.push_str(&read_license());
    result.push_str(" */\r\n");
    result.push_str("\r\n");
    result.push_str("// This file is generated by the project found in gen/ \r\n");
    result.push_str("\r\n");
    result.push_str("use serde::{Serialize, Deserialize};\r\n");
    result.push_str("\r\n");
    result.push_str("pub trait TableTrait {\r\n");
    result.push_str("    fn name() -> &'static str;\r\n");
    result.push_str("}\r\n");
    result.push_str("\r\n");
    
    let mut tables_result = String::new();
    let mut table_names :Vec<String>= vec!();
    for class_cap in class_regex.captures_iter(&data) {
        let mut class_res = String::new();
        let class = class_cap.get(0).unwrap().as_str().to_string();
        let class_name = class_cap.get(1).unwrap().as_str().to_string();

        if class_name == "Base" {
            continue;
        }

        class_res.push_str("/*\r\n");
        class_res.push_str(&class);
        class_res.push_str("*/\r\n");

        class_res.push_str("#[allow(non_snake_case)]\r\n");
        class_res.push_str("#[derive(Serialize, Deserialize, Clone)]\r\n");
        class_res.push_str(&format!("pub struct {}Store {{\r\n", class_name));
        table_names.push(class_name.to_string());
        
        println!("class {}", &class);
        let base = vec![
            Field::base("Id", "int"),
            Field::new("Created", "date"),
            Field::new("CreatedId", "string"),
            Field::new("Changed", "date"),
            Field::new("ChangedId", "string"),
            Field::new("Disabled", "boolean"),
        ];

        let mut fields:  HashMap<String, Field>= HashMap::new();
        for f in base {
            fields.insert(f.name.to_string(), f);
        }

        for field_cap in fields_regex.captures_iter(&class)  {
            let f = parse_field(field_cap.get(0).unwrap().as_str());

            fields.insert(f.name.clone(), f);
        }
        
        for (_, f) in fields {
            class_res.push_str(&f.format());
            println!("{}", &f.format());
        }

        class_res.push_str("}\r\n\r\n");

        // Add trait impl
        class_res.push_str(&format!("impl TableTrait for {}Store {{ ", class_name));
        class_res.push_str(&format!("fn name() -> &'static str {{ \"{}Store\" }}", class_name));
        class_res.push_str(" }");

        println!("{}", &class_res);
        tables_result.push_str(&class_res);
    }

    result.push_str("// Available tables:\r\n");
    for name in table_names {
        result.push_str(&format!("// -{}Store\r\n", &name));
    }
    result.push_str("\r\n");
    result.push_str(&tables_result);
    std::fs::write("next.js.txt", result).unwrap();
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

fn read_license() -> String {
    let content = std::fs::read("LICENSE").unwrap();
    String::from_utf8(content).unwrap()
}
