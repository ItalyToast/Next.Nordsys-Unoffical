use std::{io::stdin};

use next_nordsys_unoffical::{client::NClient, tables::{UserSessionStore, UserWorkOrderListStore}};
use reqwest::StatusCode;

fn main(){

    println!("Please provide your serverid, username and password to retrive your workorders");

    println!("Server ID:");
    let server_id : u64 = read_line().parse().unwrap();

    println!("Username:");
    let username = read_line();

    println!("Password:");
    let password = read_line();

    let client = NClient::login(server_id, &username, &password);
    let sessions = client.datastore::<UserSessionStore>().get_all().unwrap();
    let session = sessions.first().unwrap();
    let workorders = client.datastore::<UserWorkOrderListStore>()
        .get_filter(&format!("[[\"ResponsibleServiceId\",\"=\",{}]]", session.SessionUserId));

    match workorders {
        Ok(wos) => {
            println!("count = {}", wos.len());
            for wo in wos {
                println!("{}", serde_json::to_string_pretty(&wo).unwrap());
            }
        },
        Err(ex) => {
            println!("status: {} {}", ex.status().unwrap_or(StatusCode::OK), ex.to_string());
        },
    }
}

fn read_line() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}