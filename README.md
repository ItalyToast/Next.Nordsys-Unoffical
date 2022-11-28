# Next.Nordsys-Unoffical
An unoffical API for "next.nordsys.se".

⚠️**This code is not yet production ready, please use with caution**⚠️

# Usage

```
// using the api part
let client = NClient::login(server_id, username, password);
let invoice = client.api().supplier_invoice_document(supplier_invoice_id);
```
```
// using the datastore part
let client = NClient::login(server_id, username, password);
let settings = client.datastore::<OptionValueStore>().get_all();
```

Replace **OptionValueStore** with any of the available tables in **tables.rs**


