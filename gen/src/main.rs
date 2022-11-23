use reqwest::{header, blocking::Client};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        println!("Please provide a name");
    }

    gen_from_json_file(args.first().unwrap());
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
