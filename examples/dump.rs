use next_nordsys_unoffical::{tables::*, utils, client::NClient};
use serde::Serialize;

fn main() {
    println!("Please provide your serverid, username and password in credentials.txt");
    println!("Your serverid is found in the url: https://next.nordsys.se/**server_id**/");

    let (server_id, username, password) = utils::get_test_credentials();

    let client = NClient::login(server_id, &username, &password).unwrap();

    std::fs::create_dir_all("dump").unwrap();

    println!("dumping get_all()");
    dump(client.datastore::<AccountChartStore>().get_all());
    dump(client.datastore::<AdditionalWorkOrderStatusStore>().get_all());
    dump(client.datastore::<AdditionalWorkOrderStore>().get_all());
    dump(client.datastore::<ArticleCategoryStore>().get_all());
    dump(client.datastore::<ConstructionTypeStore>().get_all());
    dump(client.datastore::<CurrencyStore>().get_all());
    dump(client.datastore::<CustomerListStore>().get_all());
    dump(client.datastore::<CustomerStore>().get_all());
    dump(client.datastore::<CustomerTypeStore>().get_all());
    dump(client.datastore::<FormValidationStore>().get_all());
    dump(client.datastore::<GroupStore>().get_all());
    dump(client.datastore::<ItemUnitStore>().get_all());
    dump(client.datastore::<MarkupModelStore>().get_all());
    dump(client.datastore::<OfficeCompanyStore>().get_all());
    dump(client.datastore::<OptionValueStore>().get_all());
    dump(client.datastore::<ProfessionItemStore>().get_all());
    dump(client.datastore::<ProjectLimitedStore>().get_all());
    dump(client.datastore::<ProjectListStore>().get_all());
    dump(client.datastore::<ProjectStatusStore>().get_all());
    dump(client.datastore::<ProjectTreeStore>().get_all());
    dump(client.datastore::<ProjectTypeStore>().get_all());
    dump(client.datastore::<SettingStore>().get_all());
    dump(client.datastore::<SupplierStore>().get_all());
    dump(client.datastore::<UserAccessStore>().get_all());
    dump(client.datastore::<UserFavoriteListStore>().get_all());
    dump(client.datastore::<UserListStore>().get_all());
    dump(client.datastore::<UserRightStore>().get_all());
    dump(client.datastore::<UserSessionStore>().get_all());
    dump(client.datastore::<VatStore>().get_all());
    dump(client.datastore::<WorkOrderRowStore>().get_all());
    dump(client.datastore::<WorkOrderStatusStore>().get_all());
    dump(client.datastore::<WorkOrderStore>().get_all());

    
    // dump(client.datastore::<UserWorkOrderListStore>().get_all());
    // dump(client.datastore::<AdditionalWorkOrderRowStore>().get_all());
    // dump(client.datastore::<ArticleCustomerStore>().get_all());
    // dump(client.datastore::<ArticleStore>().get_all());
    // dump(client.datastore::<ChecklistDiaryStore>().get_all());
    // dump(client.datastore::<ChecklistRowStore>().get_all());
    // dump(client.datastore::<CostStore>().get_all());
    // dump(client.datastore::<CustomerContactStore>().get_all());
    // dump(client.datastore::<CustomerPricelistItemStore>().get_all());
    // dump(client.datastore::<DiaryStore>().get_all());
    // dump(client.datastore::<ExternalWorkOrderStatusStore>().get_all());
    // dump(client.datastore::<FactoryPricelistStore>().get_all());
    // dump(client.datastore::<FactoryArticleItemStore>().get_all());
    // dump(client.datastore::<FrameworkContractStore>().get_all());
    // dump(client.datastore::<FrameworkArticleItemStore>().get_all());
    // dump(client.datastore::<HourlyRateStore>().get_all());
    // dump(client.datastore::<InfoMessageStore>().get_all());
    // dump(client.datastore::<InfoNoteStore>().get_all());
    // dump(client.datastore::<InvoiceRowStore>().get_all());
    // dump(client.datastore::<InvoiceStore>().get_all());
    // dump(client.datastore::<MainMenuStore>().get_all());
    // dump(client.datastore::<MarkupModelItemStore>().get_all());
    // dump(client.datastore::<TimeStore>().get_all());
    // dump(client.datastore::<PhraseStore>().get_all());
    // dump(client.datastore::<PriceTypeStore>().get_all());
    // dump(client.datastore::<WorkOrderDocumentStore>().get_all());
    // dump(client.datastore::<ProjectEconomyBudgetStore>().get_all());
    // dump(client.datastore::<ProjectEconomyStore>().get_all());
    // dump(client.datastore::<ProjectOverviewStore>().get_all());
    // dump(client.datastore::<ProjectPricelistItemStore>().get_all());
    // dump(client.datastore::<ProjectPricelistStore>().get_all());
    // dump(client.datastore::<ProjectStatusHistoryStore>().get_all());
    // dump(client.datastore::<ProjectStore>().get_all());
    // dump(client.datastore::<ResourceStore>().get_all());
    // dump(client.datastore::<RevenueStore>().get_all());
    // dump(client.datastore::<ServiceCategoryStore>().get_all());
    // dump(client.datastore::<StaffListStore>().get_all());
    // dump(client.datastore::<StatusHistoryStore>().get_all());
    // dump(client.datastore::<TagStore>().get_all());
    // dump(client.datastore::<UserPoolWorkOrderListStore>().get_all());
    // dump(client.datastore::<UserWorkOrderListStore>().get_all());
    // dump(client.datastore::<UserProjectWorkOrderListStore>().get_all());
    // dump(client.datastore::<UserSettingStore>().get_all());
    // dump(client.datastore::<WorkOrderAssignedLocationStore>().get_all());
    // dump(client.datastore::<WorkOrderListStore>().get_all());
    // dump(client.datastore::<WorkOrderLocationStore>().get_all());
    // dump(client.datastore::<WorkOrderStatusRelationsStore>().get_all());
    // dump(client.datastore::<StaffLogStore>().get_all());
    // dump(client.datastore::<StaffPreviousDayStore>().get_all());
    // dump(client.datastore::<WorkOrderContactStore>().get_all());
    // dump(client.datastore::<UserStore>().get_all());
    // dump(client.datastore::<CustomerContactListStore>().get_all());
    // dump(client.datastore::<UserFavoriteArticleStore>().get_all());
    // dump(client.datastore::<CustomerFavoriteArticleStore>().get_all());
    // dump(client.datastore::<FavoriteListStore>().get_all());
    // dump(client.datastore::<FavoriteArticleStore>().get_all());
}

fn dump<TABLE : Serialize + TableTrait>(data : reqwest::Result<Vec<TABLE>>) {
    println!("Dumping: {}", TABLE::name());

    match &data {
        Ok(result) => {
            let json = serde_json::to_string_pretty(result).unwrap();
            std::fs::write(format!("dump/{}.json", TABLE::name()), json).unwrap();
        } ,
        Err(ex) => println!("Exception: {}", ex.to_string()),
    }
}