use std::{collections::HashMap, marker::PhantomData};

use reqwest::{blocking::{Client, ClientBuilder}, header};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::{api::{ApiResponse, ApiContext}, workorder::WorkOrderContext, project::ProjectContext, utils::params_dc_only, datastore::DatastoreContext};

#[derive(Default)]
pub struct NClient {
    pub(crate) server_id : u64,
    pub(crate) client : Client,
    pub(crate) token : Option<String>,
}

impl NClient {
    pub fn login(server_id : u64, username : &str, password : &str) -> reqwest::Result<NClient> {
        let client = ClientBuilder::new()
            .connection_verbose(true)
            .build().unwrap();
    
        let form : HashMap<&str, &str> = HashMap::from([
            ("type", "json"),
            ("userlogin", username),
            ("password", password),
        ]);
        
        let response = client.post(endpoint(server_id, "api/login"))
            .form(&form)
            .query(&params_dc_only())
            .send()?
            .json::<ApiResponse<Login>>()?;
    
        if !response.success {
            println!("Log in failed: {}", &response.errors.as_ref().unwrap());
            println!("{:?}", serde_json::to_string_pretty(&response));
        }

        let token = response.rows.unwrap().first().unwrap().Sid.clone();
    
        Ok(NClient {
            client: client,
            server_id: server_id,
            token: Some(token),
        })
    }

    /// all the /data/store functions found in the api
    /// 
    /// ## Usage
    /// 
    /// ```
    /// let items = client.datastore::<TableName>().get_all().unwrap()
    /// ```
    pub fn datastore<T>(&self) -> DatastoreContext<T> {
        DatastoreContext { 
            client: &self, 
            table: PhantomData,
        }
    }

    /// all the /api functions found in the api
    pub fn api(&self) -> ApiContext {
        ApiContext { client: self }
    }

    /// get data associated with a workorder
    pub fn workorder(&self, id : String) -> WorkOrderContext {
        WorkOrderContext {
            client: self,
            workorder_id: id,
        }
    }

    pub fn project(&self, id : String) -> ProjectContext {
        ProjectContext { 
            client: self, 
            project_id: id 
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

