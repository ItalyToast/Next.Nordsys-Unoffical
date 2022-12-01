use std::{time::{SystemTime, UNIX_EPOCH}};

pub fn params_dc_only() ->[(&'static str, String); 1]{
    [
        ("_dc", dc()),
    ]
}

pub fn params(page : i32, start : i32, limit : i32) -> [(&'static str, String); 5]{
    [
        ("_dc", dc()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

pub fn params_filter(page : i32, start : i32, limit : i32, filter : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("filter", filter.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

pub fn params_filter_by_id(page : i32, start : i32, limit : i32, id : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("filter", format!("[[\"Id\",\"=\",{}]]",id)),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

pub fn params_order(page : i32, start : i32, limit : i32, order : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("order", order.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}
pub fn dc() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    timestamp.to_string()
}

pub fn get_test_credentials() -> (u64, String, String) {
    println!("Fetching credentials from credentials.txt");

    match std::path::Path::new("credentials.txt").exists() {
        true => {
            let creds = std::fs::read_to_string("credentials.txt").unwrap();
            let mut lines = creds.split("\r\n");
            match (lines.next(), lines.next(), lines.next()) {
                (Some(server), Some(user), Some(pass)) => {
                    let sid = server.parse().expect("[Line 0]Could not parse server_id");
                    (sid, user.to_string(), pass.to_string())
                },
                _ => panic!("Did not find 3 lines in credentials.txt. Make sure you have server id, username and password"),
            }
        },
        false => {
            std::fs::write("credentials.txt", "server_id\r\nusername\r\npassword").unwrap();
            panic!("No test credentials found. A new credentials.txt have been created.");
        }
    }
}