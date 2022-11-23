use serde::{Serialize, Deserialize};

use crate::client::NClient;
use crate::{tables::{WorkOrderStatusStore, UserWorkOrderListStore, WorkOrderStore, UserProjectWorkOrderListStore, WorkOrderDocumentStore, ProjectListStore, UserRightStore, UserSessionStore, UserAccessStore, OfficeCompanyStore, UserListStore, WorkOrderListStore, ProjectLimitedStore, ChecklistRowStore, OptionValueStore}};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success : bool,
    pub rows : Option<Vec<T>>,
}

pub fn work_order_status_store(client : &NClient) -> Vec<WorkOrderStatusStore> {
    client.datastore().get_order("[[\"StatusCode\",\"ASC\"]]")
}

pub fn user_work_order_list_store(client : &NClient) -> Vec<UserWorkOrderListStore> {
    client.datastore().get_filter("[[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"Addition\",\"=\",false],[\"ResponsibleServiceId\",\"=\",235]]")
}

pub fn work_order_store(client : &NClient, id: &str) -> Vec<WorkOrderStore> {
    client.datastore().get_filter(&format!("[[\"Id\",\"=\",{}]]", id))
}

pub fn user_project_work_order_list_store(client : &NClient, id : String) -> Vec<UserProjectWorkOrderListStore> {
    client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", id))
}

pub fn work_order_document_store(client : &NClient, id : String) -> Vec<WorkOrderDocumentStore> {
    client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}],[\"HideInPDA\",\"=\",false]]", id))
}

pub fn project_list_store(client : &NClient) -> Vec<ProjectListStore> {
    client.datastore().get_filter("[[\"StatusCode\",\">=\",10],[\"StatusCode\",\"<\",90]]")
}

pub fn user_right_store(client : &NClient) -> Vec<UserRightStore> {
    client.datastore().get_all()
}

pub fn work_order_document(client : &NClient, document_id : &str) -> Vec<u8> {
    client.api().work_order_document(document_id).unwrap()
}

pub fn search_work_order(client : &NClient, query : &str) -> Vec<WorkOrderListStore> {
    client.datastore().get_filter(&format!("[[\"ProjectId\",\"!=\",0],[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"WorkOrderStatusCode\",\"<=\",89],{}]", query))
}

pub fn user_session_store(client : &NClient) -> Vec<UserSessionStore> {
    client.datastore().get_all()
}

pub fn user_access_store(client : &NClient) -> Vec<UserAccessStore> {
    client.datastore().get_all()
}

pub fn office_company_store(client : &NClient) -> Vec<OfficeCompanyStore> {
    client.datastore().get_all()
}

pub fn user_list_store(client : &NClient) -> Vec<UserListStore> {
    client.datastore().get_all()
}

pub fn project_limited_store (client : &NClient, id : &str) -> Option<ProjectLimitedStore> {
    client.datastore().get_by_id(id)
}

pub fn checklist_row_store(client : &NClient, work_order_id : &str) -> Vec<ChecklistRowStore> {
    client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}]]", work_order_id))
}

pub fn option_value_store(client : &NClient) -> Vec<OptionValueStore> {
    client.datastore().get_all()
}