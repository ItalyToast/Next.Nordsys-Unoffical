use std::{marker::PhantomData, collections::HashMap};

use reqwest::header;
use serde::de::DeserializeOwned;

use crate::{client::NClient, tables::TableTrait, api::ApiResponse, utils::{params_filter_by_id, params, params_filter, params_order, params_dc_only}};

/// Used to access the data store
pub struct DatastoreContext<'a, TABLE> {
    pub(crate) client : &'a NClient,
    pub(crate) table : PhantomData<TABLE>,
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

    ///filter:
    ///[{"property":"Id","value":10}]
    ///json:
    ///{"Checked":false,"CheckDate":null,"Id":10}
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

fn endpoint(server_id : u64, url : &str) -> String {
    format!("https://next.nordsys.se/{}/cgi/me.cgi/{}", server_id, url)
}