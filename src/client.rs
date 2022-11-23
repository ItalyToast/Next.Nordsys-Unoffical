use std::collections::HashMap;

use reqwest::{blocking::{Client, Response}, header};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::api::{self, ApiResponse};

pub struct NClient {
    server_id : u64,
    client : Client,
    token : Option<String>,
}

impl NClient {
    pub fn login(server_id : u64, username : &str, password : &str) -> NClient {
        let client = Client::new();
    
        let form : HashMap<&str, &str> = HashMap::from([
            ("type", "json"),
            ("userlogin", username),
            ("password", password),
        ]);
        
        let response = client.post(endpoint(server_id, "api/login?_dc=1668982911141"))
            .form(&form)
            .send().unwrap()
            .json::<ApiResponse<Login>>().unwrap();
    
        let token = response.rows.unwrap().first().unwrap().Sid.clone();
    
        NClient {
            client: client,
            server_id: server_id,
            token: Some(token),
        }
    }

    pub fn get_cookie(&self) -> String {
        format!("milltimesessionid={}; userName=", &self.token.as_ref().unwrap())
    }

    pub fn get<Q: Serialize + ?Sized, R : DeserializeOwned>(&self, url : &str, query : &Q) -> reqwest::Result<ApiResponse<R>> {
        self.client.get(endpoint(self.server_id, url))
            .header(header::COOKIE, self.get_cookie())
            .query(query)
            .send()?
            .json()
    }

    pub fn get_binary(&self, url : &str) -> reqwest::Result<Vec<u8>> {
        let mut response = self.client.get(endpoint(self.server_id, url))
            .header(header::COOKIE, self.get_cookie())
            .send()?;

        let mut data :Vec<u8>= Vec::new();
        response.copy_to(&mut data).unwrap();
        Ok(data)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Login {
    MobileClient : String,
    Sid : String,
}

fn endpoint(server_id : u64, url : &str) -> String {
    format!("https://next.nordsys.se/{}/cgi/me.cgi/{}", server_id, url)
}