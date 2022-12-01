use regex::RegexBuilder;

pub fn run_codegen() {
    let data = std::fs::read_to_string("lang").unwrap();
    let lang_regex = RegexBuilder::new("MT\\.setTR\\(\"(.*?)\"(.*?)\\[null\\]\\]\\);")
        .dot_matches_new_line(true)
        .build().unwrap();
    let entry_regex = RegexBuilder::new("\\[\"([^\"].*?)\",\"([^\"].*?)\"\\]")
        .build().unwrap();

        std::fs::create_dir("langpacks").unwrap();

    for lang in lang_regex.captures_iter(&data) {
        let lang_code = lang.get(1).unwrap().as_str();
        let body = lang.get(2).unwrap().as_str();
        let mut result = String::new();

        for entry in entry_regex.captures_iter(body) {
            let key = entry.get(1).unwrap().as_str();
            let value = entry.get(2).unwrap().as_str();
            
            result.push_str(&format!("\"{}\", \"{}\"\r\n", key, value));
            println!("[{}]{} : {}", lang_code, key, value);
        }

        std::fs::write(format!("langpacks/{}.langpack", lang_code), result).unwrap();
    }
}