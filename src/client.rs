use std::{collections::HashMap, marker::PhantomData};

use reqwest::{blocking::{Client, Response}, header};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::{api::ApiResponse, tables_trait::TableTrait};

#[derive(Default)]
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

    pub fn datastore<T>(&self) -> DatastoreContext<T> {
        DatastoreContext { 
            client: &self, 
            table: PhantomData,
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

    /// all the /api functions found in the api
    pub fn api(&self) -> ApiContext {
        ApiContext { client: self }
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

/// Contains all /api endpoints
pub struct ApiContext<'a> {
    client : &'a NClient,
}

impl ApiContext<'_> {
    fn endpoint(&self, name : &str) -> String {
        format!("https://next.nordsys.se/{}/cgi/me.cgi/api/{}", self.client.server_id, name)
    }

    pub fn supplier_invoice_document(&self, supplier_invoice_id : String) -> reqwest::Result<()> {
        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "binary".to_string()),
            ("SupplierInvoiceId", supplier_invoice_id.to_string()),
        ];

        let _response = self.client.client.get(self.endpoint("supplierInvoiceDocument"))
            .query(&params)
            .send();
        
        todo!("Not yet implemented: missing json body");
    }

    pub fn send_work_order_msg(&self, my_awo_id : String, my_document_id : String, to_customer : bool) -> reqwest::Result<()>{
        let to_customer_str = match to_customer {
            false => "0".to_string(),
            true => "1".to_string(),
        };
        
        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "json".to_string()),
            ("msgtype", "email".to_string()),
            ("workorderid", my_awo_id),
            ("documentid", my_document_id),
            ("tocustomer", to_customer_str),
        ];

        let _response = self.client.client.post(self.endpoint("sendWorkOrderMsg"))
            .query(&params)
            .send();
        
        todo!("Not yet implemented: missing json body");
    }

    pub fn send_work_order_document(&self, document_id : String, customer_contact_id : String) -> reqwest::Result<()> {
        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "json".to_string()),
            ("msgtype", "email".to_string()),
            ("workorderdocumentid", document_id),
            ("customercontactid", customer_contact_id),
        ];
            
        let _response = self.client.client.post(self.endpoint("sendWorkOrderMsg"))
            .query(&params)
            .send();
            
        todo!("Not yet implemented: missing json body");
    }

    pub fn send_work_order_info_msg(&self, workorder_id : String, customer_contact_id : String) -> reqwest::Result<()>  {
        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "json".to_string()),
            ("msgtype", "email".to_string()),
            ("workorderid", workorder_id),
            ("customercontactid", customer_contact_id),
        ];
            
        let _response = self.client.client.post(self.endpoint("sendWorkOrderMsg"))
            .query(&params)
            .send();
            
        todo!("Not yet implemented: missing json body");
    }

    pub fn copy_diary(&self) {
        todo!("Not yet implemented");
    }

    pub fn get_report(&self) {
        todo!("Not yet implemented");
    }

    pub fn update_time_cost(&self) {
        todo!("Not yet implemented");
    }

    pub fn upload_work_order_document(&self) {
        todo!("Not yet implemented");
    }

    pub fn work_order_document(&self, document_id : &str)  -> reqwest::Result<Vec<u8>>{
        self.client.get_binary(&format!("api/workOrderDocument?type=binary&documentid={}", document_id))
    }
    
    pub fn impersonate(&self) -> reqwest::Result<Response>
    {
        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "json".to_string()),
            ("userId", "0".to_string()),
        ];

        let response = self.client.client.get(self.endpoint("impersonate"))
            .query(&params)
            .send();

        response
    }

    pub fn send_order_summary(&self, workorder_id : String, user_id : String, show_price : bool, show_vat : bool) -> reqwest::Result<Response> {
        let sp = match show_price {
            false => "0",
            true => "1",
        };
        let sv = match show_vat {
            false => "0",
            true => "1",
        };

        let params = [
            ("_dc", "1668982911141".to_string()),
            ("type", "json".to_string()),
            ("msgtype", "email".to_string()),
            ("workorderid", workorder_id),
            ("userid", user_id),
            ("showprice", sp.to_string()),
            ("showvat", sv.to_string()),
        ];

        let response = self.client.client.get(self.endpoint("impersonate"))
            .query(&params)
            .send();

        todo!("Not yet implemented: missing json body");
    }

    pub fn copy_work_order(){
        todo!("TODO: AJAX stuff happening here");
    }
}

/// Used to access the data store
pub struct DatastoreContext<'a, TABLE> {
    client : &'a NClient,
    table : PhantomData<TABLE>,
}

impl<TABLE> DatastoreContext<'_, TABLE>
    where TABLE : DeserializeOwned + TableTrait + Clone {
    fn endpoint() -> String {
        format!("data/store/{}", TABLE::name())
    }

    pub fn get_by_id(&self, id: &str) -> Option<TABLE> {
        let params = params_filter_by_id(1, 0, 0, id);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params).unwrap();
        let single = response.rows?.first().unwrap().clone();
        Some(single)
    }

    pub fn get_all(&self) -> Vec<TABLE> {
        let params = params(1, 0, 0);
        let response = self.client.get(&Self::endpoint(), &params);
        response.unwrap().rows.unwrap()
    }

    pub fn get_single(&self, id: &str) -> Option<TABLE> {
        let params = params_filter_by_id(1, 0, 0, id);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params).unwrap();
        let single = response.rows?.first().unwrap().clone();
        Some(single)
    }

    pub fn get_filter(&self, filter: &str) -> Vec<TABLE> {
        let params = params_filter(1, 0, 0, filter);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params).unwrap();
        response.rows.unwrap()
    }

    pub fn get_order(&self, order: &str) -> Vec<TABLE> {
        let params = params_order(1, 0, 0, order);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params).unwrap();
        response.rows.unwrap()
    }

    pub fn put(&self, json : &str, filter: &str) -> reqwest::Result<ApiResponse<TABLE>> {
        let form : HashMap<&str, &str> = HashMap::from([
            ("type", "json"),
            ("filter", filter),
            ("json", json),
        ]);

        self.client.client.put(endpoint(self.client.server_id, &Self::endpoint()))
            .header(header::COOKIE, self.client.get_cookie())
            .query(&dc_only())
            .form(&form)
            .send()?
            .json()
    }

    pub fn delete(&self, json : &str, filter: &str) -> reqwest::Result<ApiResponse<TABLE>> {
        let form : HashMap<&str, &str> = HashMap::from([
            ("type", "json"),
            ("filter", filter),
            ("json", json),
        ]);

        self.client.client.put(endpoint(self.client.server_id, &Self::endpoint()))
            .header(header::COOKIE, self.client.get_cookie())
            .query(&dc_only())
            .form(&form)
            .send()?
            .json()
    }
}

fn dc_only() ->[(&'static str, String); 1]{
    [
        ("_dc", "1668982911141".to_string()),
    ]
}

fn params(page : i32, start : i32, limit : i32) -> [(&'static str, String); 5]{
    [
        ("_dc", "1668982911141".to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_filter(page : i32, start : i32, limit : i32, filter : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", "1668982911141".to_string()),
        ("filter", filter.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_filter_by_id(page : i32, start : i32, limit : i32, id : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", "1668982911141".to_string()),
        ("filter", format!("[[\"Id\",\"=\",{}]]",id)),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_order(page : i32, start : i32, limit : i32, order : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", "1668982911141".to_string()),
        ("order", order.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}