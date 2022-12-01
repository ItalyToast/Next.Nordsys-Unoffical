use reqwest::blocking::Response;
use serde::{Serialize, Deserialize};

use crate::client::NClient;
use crate::utils::dc;
use crate::{tables::{WorkOrderStatusStore, UserWorkOrderListStore, UserProjectWorkOrderListStore, WorkOrderDocumentStore, ProjectListStore, UserRightStore, UserSessionStore, UserAccessStore, OfficeCompanyStore, UserListStore, WorkOrderListStore}};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success : bool,
    pub rows : Option<Vec<T>>,
    pub errors : Option<String>,
    pub error_code : Option<i32>,
}

/// Contains all /api endpoints
pub struct ApiContext<'a> {
    pub client : &'a NClient,
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

pub fn work_order_status_store(client : &NClient) -> Vec<WorkOrderStatusStore> {
    client.datastore().get_order("[[\"StatusCode\",\"ASC\"]]").unwrap()
}

pub fn user_work_order_list_store(client : &NClient, user_id : &str) -> Vec<UserWorkOrderListStore> {
    client.datastore().get_filter(&format!("[[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"Addition\",\"=\",false],[\"ResponsibleServiceId\",\"=\",{}]]", user_id)).unwrap()
}

pub fn user_project_work_order_list_store(client : &NClient, id : String) -> Vec<UserProjectWorkOrderListStore> {
    client.datastore().get_filter(&format!("[[\"ProjectId\",\"=\",{}]]", id)).unwrap()
}

pub fn work_order_document_store(client : &NClient, id : String) -> Vec<WorkOrderDocumentStore> {
    client.datastore().get_filter(&format!("[[\"WorkOrderId\",\"=\",{}],[\"HideInPDA\",\"=\",false]]", id)).unwrap()
}

pub fn project_list_store(client : &NClient) -> Vec<ProjectListStore> {
    client.datastore().get_filter("[[\"StatusCode\",\">=\",10],[\"StatusCode\",\"<\",90]]").unwrap()
}

pub fn user_right_store(client : &NClient) -> Vec<UserRightStore> {
    client.datastore().get_all().unwrap()
}

pub fn search_work_order(client : &NClient, query : &str) -> Vec<WorkOrderListStore> {
    client.datastore().get_filter(&format!("[[\"ProjectId\",\"!=\",0],[\"ProjectStatusCode\",\">=\",10],[\"ProjectStatusCode\",\"<\",90],[\"WorkOrderStatusCode\",\"<=\",89],{}]", query)).unwrap()
}

pub fn user_session_store(client : &NClient) -> Vec<UserSessionStore> {
    client.datastore().get_all().unwrap()
}

pub fn user_access_store(client : &NClient) -> Vec<UserAccessStore> {
    client.datastore().get_all().unwrap()
}

pub fn office_company_store(client : &NClient) -> Vec<OfficeCompanyStore> {
    client.datastore().get_all().unwrap()
}

pub fn user_list_store(client : &NClient) -> Vec<UserListStore> {
    client.datastore().get_all().unwrap()
}

pub fn option_value_store(client : &NClient) -> Vec<crate::tables::OptionValueStore> {
    client.datastore().get_all().unwrap()
}