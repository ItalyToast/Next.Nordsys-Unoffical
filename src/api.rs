use std::{collections::HashMap};

use reqwest::{self, blocking::{Client}};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::client::NClient;
use crate::{tables::{WorkOrderStatusStore, UserWorkOrderListStore, WorkOrderStore, UserProjectWorkOrderListStore, WorkOrderDocumentStore, ProjectListStore, UserRightStore, UserSessionStore, UserAccessStore, OfficeCompanyStore, UserListStore, WorkOrderListStore, ProjectLimitedStore, ChecklistRowStore, OptionValueStore}};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success : bool,
    pub rows : Option<Vec<T>>,
}

fn datastore(url : &str) -> String {
    format!("data/store/{}", url)
}

pub fn work_order_status_store(client : &NClient) -> Vec<WorkOrderStatusStore> {
    let params = params_order(1, 0, 0, "[[\"StatusCode\",\"ASC\"]]");
    let response = client.get(&datastore("WorkOrderStatusStore"), &params);
    
    response.unwrap().rows.unwrap()
}

pub fn user_work_order_list_store(client : &NClient) -> Vec<UserWorkOrderListStore> {
    let params = params_filter(1, 0, 0, "[[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"Addition\",\"=\",false],[\"ResponsibleServiceId\",\"=\",235]]");

    let response = client.get(&datastore("UserWorkOrderListStore"), &params); 
    response.unwrap().rows.unwrap()
}

pub fn work_order_store(client : &NClient, id: &str) -> Vec<WorkOrderStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"Id\",\"=\",{}]]", id));
    
    let response = client.get(&datastore("WorkOrderStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn user_project_work_order_list_store(client : &NClient) -> Vec<UserProjectWorkOrderListStore> {
    let params = params_filter(1, 0, 0, "[[\"ProjectId\",\"=\",85]]");

    let response = client.get(&datastore("UserProjectWorkOrderListStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn work_order_document_store(client : &NClient) -> Vec<WorkOrderDocumentStore> {
    let params = params_filter(1, 0, 0, "[[\"WorkOrderId\",\"=\",103],[\"HideInPDA\",\"=\",false]]");

    let response = client.get(&datastore("WorkOrderDocumentStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn project_list_store(client : &NClient) -> Vec<ProjectListStore> {
    let params = params_filter(1, 0, 0, "[[\"StatusCode\",\">=\",10],[\"StatusCode\",\"<\",90]]");

    let response = client.get(&datastore("ProjectListStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn user_right_store(client : &NClient) -> Vec<UserRightStore> {
    get_all(&datastore("UserRightStore"), &client)
}

pub fn work_order_document(client : &NClient, document_id : &str) -> Vec<u8> {
    client.get_binary(&format!("api/workOrderDocument?type=binary&documentid={}", document_id)).unwrap()
}

pub fn search_work_order(client : &NClient, query : &str) -> Vec<WorkOrderListStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"ProjectId\",\"!=\",0],[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"WorkOrderStatusCode\",\"<=\",89],{}]", query));

    let response = client.get(&datastore("WorkOrderListStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn user_session_store(client : &NClient) -> Vec<UserSessionStore> {
    get_all(&datastore("UserSessionStore"), client)
}

pub fn user_access_store(client : &NClient) -> Vec<UserAccessStore> {
    get_all(&datastore("data/store/UserAccessStore"), client)
}

pub fn office_company_store(client : &NClient) -> Vec<OfficeCompanyStore> {
    get_all(&datastore("OfficeCompanyStore"), client)
}

pub fn user_list_store(client : &NClient) -> Vec<UserListStore> {
    get_all(&datastore("UserListStore"), client)
}

pub fn project_limited_store (client : &NClient, id : &str) -> Vec<ProjectLimitedStore> {
    let params = params_filter_by_id(1, 0, 0, &id);
    let response = client.get(&datastore("ProjectLimitedStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn checklist_row_store(client : &NClient, work_order_id : &str) -> Vec<ChecklistRowStore> {
    let params = params_filter(1, 0, 0, &format!("[[\"WorkOrderId\",\"=\",{}]]", work_order_id));
    let response = client.get(&datastore("ChecklistRowStore"), &params);
    response.unwrap().rows.unwrap()
}

pub fn option_value_store(client : &NClient) -> Vec<OptionValueStore> {
    get_all(&datastore("OptionValueStore"), client)
}

fn get_all<Table>(url : &str, client : &NClient) -> Vec<Table> 
    where Table : DeserializeOwned {
    let params = params(1, 0, 0);
    
    let response = client.get(url, &params);

    response.unwrap().rows.unwrap()
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