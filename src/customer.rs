use crate::{tables, client::NClient};

pub struct CustomerContext<'a> {
    client : &'a NClient,
    customer_id : String,
} 

impl CustomerContext<'_> {
    pub fn additional_work_order(&self) -> reqwest::Result<Vec<tables::AdditionalWorkOrderStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn article_customer(&self) -> reqwest::Result<Vec<tables::ArticleCustomerStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn customer_contact(&self) -> reqwest::Result<Vec<tables::CustomerContactStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn customer_pricelist_item(&self) -> reqwest::Result<Vec<tables::CustomerPricelistItemStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn project_list(&self) -> reqwest::Result<Vec<tables::ProjectListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn project_overview(&self) -> reqwest::Result<Vec<tables::ProjectOverviewStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn project_pricelist_item(&self) -> reqwest::Result<Vec<tables::ProjectPricelistItemStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn project(&self) -> reqwest::Result<Vec<tables::ProjectStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn staff_list(&self) -> reqwest::Result<Vec<tables::StaffListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn user_project_work_order_list(&self) -> reqwest::Result<Vec<tables::UserProjectWorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn work_order_list(&self) -> reqwest::Result<Vec<tables::WorkOrderListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn work_order(&self) -> reqwest::Result<Vec<tables::WorkOrderStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn work_order_contact_store(&self) -> reqwest::Result<Vec<tables::WorkOrderContactStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn customer_contact_list(&self) -> reqwest::Result<Vec<tables::CustomerContactListStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn project_limited(&self) -> reqwest::Result<Vec<tables::ProjectLimitedStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }

    pub fn customer_favorite_article(&self) -> reqwest::Result<Vec<tables::CustomerFavoriteArticleStore>> {  
        self.client.datastore().get_filter(&format!("[[\"CustomerId\",\"=\",{}]]", self.customer_id))
    }
}