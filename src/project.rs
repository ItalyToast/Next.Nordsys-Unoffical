use crate::{tables, client::NClient};

pub struct ProjectContext<'a> {
    pub client : &'a NClient,
    pub project_id : String,
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