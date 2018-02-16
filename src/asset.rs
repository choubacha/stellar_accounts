
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
   pub asset_code: String,
   pub asset_issuer: String,
   pub num_accounts: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embedded<T> {
    _embedded: Records<T>
}

#[derive(Serialize, Deserialize, Debug)]
struct Records<T> {
    records: Vec<T>
}

pub fn get() -> Asset {
    let client = reqwest::Client::new();
    let url = "https://horizon.stellar.org";
    let asset_code = "MOBI";
    let asset_issuer = "GBNMSB7UXFOGWC3E6BQRKYOKJCAYUL6WBZMCINHKGWAD3PTRXNWUQ2BB";
    let mut response = client.get(&format!("{}/assets?asset_code={}&asset_issuer={}", url, asset_code, asset_issuer))
        .send()
        .unwrap();
    let json_str = response.text().expect("there was no body");
    println!("Mobi is: {}", json_str);
    let assets: Vec<Asset> = serde_json::from_str::<Embedded<Asset>>(&json_str)
        .expect("invalid json")._embedded.records;
    assets.remove(0)
}
