use next_nordsys_unoffical::{client::NClient, tables::{UserSessionStore, UserWorkOrderListStore}};
use next_nordsys_unoffical::utils;
use reqwest::StatusCode;

fn main(){

    println!("Please provide your serverid, username and password in credentials.txt");
    println!("Your serverid is found in the url: https://next.nordsys.se/**server_id**/");

    let (server_id, username, password) = utils::get_test_credentials();

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