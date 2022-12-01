use crate::{tables, client::NClient};

pub struct WorkOrderContext<'a> {
    pub client : &'a NClient,
    pub workorder_id : String,
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