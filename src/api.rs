use std::{collections::HashMap};

use reqwest::{self, blocking::{Client, RequestBuilder, Response}, header, };
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::next_api::tables::{WorkOrderStatusStore, UserWorkOrderListStore, WorkOrderStore, UserProjectWorkOrderListStore, WorkOrderDocumentStore, ProjectListStore, UserRightStore, UserSessionStore, UserAccessStore, OfficeCompanyStore, UserListStore, WorkOrderListStore, ProjectLimitedStore, ChecklistRowStore, OptionValueStore};

static HOST : &str = "https://next.nordsys.se/300153/cgi/me.cgi/";

#[derive(Default, Debug)]
pub struct LoginToken {
    token : String,
}

impl LoginToken {
    fn get_cookie(&self) -> String {
        format!("milltimesessionid={}; userName=", &self.token)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success : bool,
    pub rows : Option<Vec<T>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Login {
    MobileClient : String,
    Sid : String,
}

fn endpoint(url : &str) -> String {
    let mut result = HOST.to_string();
    result.push_str(url);
    result
}

pub fn login(client : &Client, username : &str, password : &str) -> LoginToken {
    //type=json&userlogin=user&password=pass
    let form : HashMap<&str, &str> = HashMap::from([
        ("type", "json"),
        ("userlogin", username),
        ("password", password),
    ]);
    
    let response = client.post(endpoint("api/login?_dc=1668982911141"))
        .form(&form)
        .send().unwrap()
        .json::<ApiResponse<Login>>().unwrap();

    LoginToken { token: response.rows.unwrap().first().unwrap().Sid.clone() }
}

//https://next.nordsys.se/300153/cgi/me.cgi/data/store/WorkOrderStatusStore?_dc=1668986551077&type=json&page=1&start=0&limit=0&order=[["StatusCode","ASC"]]
pub fn work_order_status_store(client : &Client, token : &LoginToken) -> Vec<WorkOrderStatusStore> {
    let params = params_order(1, 0, 0, "[[\"StatusCode\",\"ASC\"]]");
    
    let request = client.get(endpoint("data/store/WorkOrderStatusStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params);

    let response = send(request);

    match response.json::<ApiResponse<WorkOrderStatusStore>>() {
        Ok(r) => r.rows.unwrap(),
        Err(ex) => {
            println!("{:?}", ex);
            todo!();
        },
    }
}

pub fn user_work_order_list_store(client : &Client, token : &LoginToken) -> Vec<UserWorkOrderListStore> {
    let params = params_filter(1, 0, 0, "[[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"Addition\",\"=\",false],[\"ResponsibleServiceId\",\"=\",235]]");

    let response = client.get("https://next.nordsys.se/300153/cgi/me.cgi/data/store/UserWorkOrderListStore")
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap()
        .json::<ApiResponse<UserWorkOrderListStore>>().unwrap();

    response.rows.unwrap()
}

//https://next.nordsys.se/300153/cgi/me.cgi/data/store/WorkOrderStore?_dc=1668990319778&filter=[["Id","=",103]]&type=json&page=1&start=0&limit=0
pub fn work_order_store(client : &Client, token : &LoginToken, id: &str) -> Vec<WorkOrderStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"Id\",\"=\",{}]]", id));
    
    let response = client.get(endpoint("data/store/WorkOrderStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

//https://next.nordsys.se/300153/cgi/me.cgi/data/store/UserProjectWorkOrderListStore?_dc=1668996716229&filter=%5B%5B%22ProjectId%22%2C%22%3D%22%2C85%5D%5D&type=json&page=1&start=0&limit=0
pub fn user_project_work_order_list_store(client : &Client, token : &LoginToken) -> Vec<UserProjectWorkOrderListStore> {
    let params = params_filter(1, 0, 0, "[[\"ProjectId\",\"=\",85]]");

    let response = client.get(endpoint("data/store/UserProjectWorkOrderListStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

//https://next.nordsys.se/300153/cgi/me.cgi/data/store/WorkOrderDocumentStore?_dc=1668997222310&filter=%5B%5B%22WorkOrderId%22%2C%22%3D%22%2C103%5D%2C%5B%22HideInPDA%22%2C%22%3D%22%2Cfalse%5D%5D&type=json&page=1&start=0&limit=0
pub fn work_order_document_store(client : &Client, token : &LoginToken) -> Vec<WorkOrderDocumentStore> {
    let params = params_filter(1, 0, 0, "[[\"WorkOrderId\",\"=\",103],[\"HideInPDA\",\"=\",false]]");

    let response = client.get(endpoint("data/store/WorkOrderDocumentStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

//https://next.nordsys.se/300153/cgi/me.cgi/data/store/ProjectListStore?_dc=1668998715269&filter=%5B%5B%22StatusCode%22%2C%22%3E%3D%22%2C10%5D%2C%5B%22StatusCode%22%2C%22%3C%22%2C90%5D%5D&type=json&page=1&start=0&limit=100
pub fn project_list_store(client : &Client, token : &LoginToken) -> Vec<ProjectListStore> {
    let params = params_filter(1, 0, 0, "[[\"StatusCode\",\">=\",10],[\"StatusCode\",\"<\",90]]");

    let response = client.get(endpoint("data/store/ProjectListStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

pub fn user_right_store(client : &Client, token : &LoginToken) -> Vec<UserRightStore> {
    get_all(&endpoint("data/store/UserRightStore"), client, token)
}

//https://next.nordsys.se/300153/cgi/me.cgi/api/workOrderDocument?type=binary&documentid=2654
pub fn work_order_document(client : &Client, token : &LoginToken, document_id : &str) -> Vec<u8> {
    let mut response = client.get(endpoint(&format!("api/workOrderDocument?type=binary&documentid={}", document_id)))
        .header(header::COOKIE, token.get_cookie())
        .send().unwrap();

    let mut data = Vec::new();
    response.copy_to(&mut data).unwrap();
    data
}

pub fn search_work_order(client : &Client, token : &LoginToken, query : &str) -> Vec<WorkOrderListStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"ProjectId\",\"!=\",0],[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"WorkOrderStatusCode\",\"<=\",89],{}]", query));

    let response = client.get(endpoint("data/store/WorkOrderListStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

pub fn user_session_store(client : &Client, token : &LoginToken) -> Vec<UserSessionStore> {
    get_all(&endpoint("data/store/UserSessionStore"), client, token)
}



pub fn user_access_store(client : &Client, token : &LoginToken) -> Vec<UserAccessStore> {
    get_all(&endpoint("data/store/UserAccessStore"), client, token)
}



pub fn office_company_store(client : &Client, token : &LoginToken) -> Vec<OfficeCompanyStore> {
    get_all(&endpoint("data/store/OfficeCompanyStore"), client, token)
}



pub fn user_list_store(client : &Client, token : &LoginToken) -> Vec<UserListStore> {
    get_all(&endpoint("data/store/UserListStore"), client, token)
}

pub fn project_limited_store (client : &Client, token : &LoginToken, id : &str) -> Vec<ProjectLimitedStore> {
    let params = params_filter_by_id(1, 0, 0, &id);
    
    let response = client.get(endpoint("data/store/ProjectLimitedStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

pub fn checklist_row_store(client : &Client, token : &LoginToken, work_order_id : &str) -> Vec<ChecklistRowStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"WorkOrderId\",\"=\",{}]]", work_order_id));
    
    let response = client.get(endpoint("data/store/ChecklistRowStore"))
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap();

    parse_json(response).rows.unwrap()
}

pub fn option_value_store(client : &Client, token : &LoginToken) -> Vec<OptionValueStore> {
    get_all(&endpoint("data/store/OptionValueStore"), client, token)
}

fn get_all<Table>(url : &str, client : &Client, token : &LoginToken) -> Vec<Table> 
    where Table : DeserializeOwned
    {
    let params = params(1, 0, 0);
    
    let response = client.get(url)
        .header(header::COOKIE, token.get_cookie())
        .query(&params)
        .send().unwrap()
        .json::<ApiResponse<Table>>().unwrap();

    response.rows.unwrap()
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


fn send(request : RequestBuilder) -> Response {
    println!("{:?}", request);
    request.send().unwrap()
}

fn parse_json<T>(response :reqwest::blocking::Response) -> ApiResponse<T> where T: serde::de::DeserializeOwned {
    let str = response.text().unwrap();
    println!("{}", &str);
    match serde_json::from_str(&str) {
        Ok(json) => json,
        Err(ex) => {
            println!("{:?}", ex);
            todo!();
        },
    }
}
