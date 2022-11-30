use std::{collections::HashMap, marker::PhantomData, time::{UNIX_EPOCH, SystemTime}};

use reqwest::{blocking::{Client, Response,ClientBuilder}, header};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::{api::ApiResponse, tables_trait::TableTrait, tables::{self}};

#[derive(Default)]
pub struct NClient {
    server_id : u64,
    client : Client,
    token : Option<String>,
}

impl NClient {
    pub fn login(server_id : u64, username : &str, password : &str) -> NClient {
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
            .send().unwrap()
            .json::<ApiResponse<Login>>().unwrap();
    
        if !response.success {
            println!("Log in failed: {}", &response.errors.as_ref().unwrap());
            println!("{:?}", serde_json::to_string_pretty(&response));
        }

        let token = response.rows.unwrap().first().unwrap().Sid.clone();
    
        NClient {
            client: client,
            server_id: server_id,
            token: Some(token),
        }
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
            ("_dc", dc()),
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
            ("_dc", dc()),
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
            ("_dc", dc()),
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
            ("_dc", dc()),
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
            ("_dc", dc()),
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
            ("_dc", dc()),
            ("type", "json".to_string()),
            ("msgtype", "email".to_string()),
            ("workorderid", workorder_id),
            ("userid", user_id),
            ("showprice", sp.to_string()),
            ("showvat", sv.to_string()),
        ];

        let _response = self.client.client.get(self.endpoint("impersonate"))
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

    pub fn get_by_id(&self, id: &str) -> reqwest::Result<Option<TABLE>> {
        let params = params_filter_by_id(1, 0, 0, id);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params)?;
        let single = response.rows.unwrap().first().unwrap().clone();
        Ok(Some(single))
    }

    pub fn get_all(&self) -> reqwest::Result<Vec<TABLE>> {
        let params = params(1, 0, 0);
        let response = self.client.get(&Self::endpoint(), &params);
        Ok(response?.rows.unwrap())
    }

    pub fn get_single(&self, id: &str) -> reqwest::Result<Option<TABLE>> {
        let params = params_filter_by_id(1, 0, 0, id);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params)?;
        let single = response.rows.unwrap().first().unwrap().clone();
        Ok(Some(single))
    }

    pub fn get_filter(&self, filter: &str) -> reqwest::Result<Vec<TABLE>> {
        let params = params_filter(1, 0, 0, filter);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params)?;
        Ok(response.rows.unwrap())
    }

    pub fn get_order(&self, order: &str) -> reqwest::Result<Vec<TABLE>> {
        let params = params_order(1, 0, 0, order);
        let response : ApiResponse<TABLE> = self.client.get(&Self::endpoint(), &params)?;
        Ok(response.rows.unwrap())
    }

    pub fn put(&self, json : &str, filter: &str) -> reqwest::Result<ApiResponse<TABLE>> {
        let form : HashMap<&str, &str> = HashMap::from([
            ("type", "json"),
            ("filter", filter),
            ("json", json),
        ]);

        self.client.client.put(endpoint(self.client.server_id, &Self::endpoint()))
            .header(header::COOKIE, self.client.get_cookie())
            .query(&params_dc_only())
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
            .query(&params_dc_only())
            .form(&form)
            .send()?
            .json()
    }
}

pub struct WorkOrderContext<'a> {
    client : &'a NClient,
    workorder_id : String,
}

impl WorkOrderContext<'_> {
    pub fn get(&self) -> reqwest::Result<Option<tables::WorkOrderStore>> {
        self.client.datastore().get_by_id(&self.workorder_id)
    }
    
    ///Returns all documents in WorkOrderDocumentStore associated with the current work order.
    pub fn documents(&self) -> reqwest::Result<Vec<tables::WorkOrderDocumentStore>> {
        self.client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}]]", self.workorder_id))
    }

    ///Returns the checklist in ChecklistRowStore associated with the current work order.
    pub fn checks(&self) -> reqwest::Result<Vec<tables::ChecklistRowStore>> {
        self.client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}]]", self.workorder_id))
    }

    ///Returns the resources in WorkOrderRowStore associated with the current work order.
    pub fn rows(&self) -> reqwest::Result<Vec<tables::WorkOrderRowStore>> {
        self.client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}]]", self.workorder_id))
    }
}

pub struct ProjectContext<'a> {
    client : &'a NClient,
    project_id : String,
}

impl ProjectContext<'_> {
    pub fn time(&self) -> reqwest::Result<Vec<tables::TimeStore>> {
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn additional_work_order(&self) -> reqwest::Result<Vec<tables::AdditionalWorkOrderStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn cost(&self) -> reqwest::Result<Vec<tables::CostStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn diary(&self) -> reqwest::Result<Vec<tables::DiaryStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn work_order_document(&self) -> reqwest::Result<Vec<tables::WorkOrderDocumentStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn project_economy_budget(&self) -> reqwest::Result<Vec<tables::ProjectEconomyBudgetStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn project_economy(&self) -> reqwest::Result<Vec<tables::ProjectEconomyStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn project_overview(&self) -> reqwest::Result<Vec<tables::ProjectOverviewStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }
    
    pub fn project_pricelist_item(&self) -> reqwest::Result<Vec<tables::ProjectPricelistItemStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn project_status_history(&self) -> reqwest::Result<Vec<tables::ProjectStatusHistoryStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn revenue(&self) -> reqwest::Result<Vec<tables::RevenueStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn staff_list(&self) -> reqwest::Result<Vec<tables::StaffListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn user_pool_work_order_list(&self) -> reqwest::Result<Vec<tables::UserPoolWorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn user_work_order_list(&self) -> reqwest::Result<Vec<tables::UserWorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn user_project_work_order_list(&self) -> reqwest::Result<Vec<tables::UserProjectWorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn work_order_assigned_location(&self) -> reqwest::Result<Vec<tables::WorkOrderAssignedLocationStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn work_order_list(&self) -> reqwest::Result<Vec<tables::WorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }
    
    pub fn work_order_location(&self) -> reqwest::Result<Vec<tables::WorkOrderLocationStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn work_order(&self) -> reqwest::Result<Vec<tables::WorkOrderStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn staff_log(&self) -> reqwest::Result<Vec<tables::StaffLogStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn staff_previous_day(&self) -> reqwest::Result<Vec<tables::StaffPreviousDayStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn project_limited(&self) -> reqwest::Result<Vec<tables::ProjectLimitedStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }

    pub fn work_order_row(&self) -> reqwest::Result<Vec<tables::WorkOrderRowStore>> {  
        self.client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", self.project_id))
    }
}

fn params_dc_only() ->[(&'static str, String); 1]{
    [
        ("_dc", dc()),
    ]
}

fn params(page : i32, start : i32, limit : i32) -> [(&'static str, String); 5]{
    [
        ("_dc", dc()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_filter(page : i32, start : i32, limit : i32, filter : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("filter", filter.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_filter_by_id(page : i32, start : i32, limit : i32, id : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("filter", format!("[[\"Id\",\"=\",{}]]",id)),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}

fn params_order(page : i32, start : i32, limit : i32, order : &str) -> [(&'static str, String); 6]{
    [
        ("_dc", dc()),
        ("order", order.to_string()),
        ("type", "json".to_string()),
        ("page", page.to_string()),
        ("start", start.to_string()),
        ("limit", limit.to_string()),
    ]
}
fn dc() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    timestamp.to_string()
}