extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub account_id: String,
    pub sequence: String,
}

pub fn get() -> Account {
    let client = reqwest::Client::new();
    let mut command = String::new();
    let url = "https://horizon-testnet.stellar.org";
    let account_id = "GATUX2IIOOPQ5CWW3SFVOFUKBBSJ72GIGROR3U4PAA7VUYZT5AYD3HL4";
    let mut response = client.get(&format!("{}/accounts/{}", url, account_id))
        .send()
        .unwrap();
    let json_str = response.text().expect("there was no body");
    serde_json::from_str::<Account>(&json_str)
        .expect("invalid json")
}
