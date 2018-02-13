#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    account_id: String,
    sequence: String,
}

fn main() {
    let url = "https://horizon-testnet.stellar.org";
    let account_id = "GATUX2IIOOPQ5CWW3SFVOFUKBBSJ72GIGROR3U4PAA7VUYZT5AYD3HL4";
    let client = reqwest::Client::new();
    let mut response = client.get(&format!("{}/accounts/{}", url, account_id))
        .send()
        .unwrap();
    let json_str = response.text().expect("there was no body");
    let account: Account = serde_json::from_str::<Account>(&json_str)
        .expect("invalid json");
    println!("id:          {}", account.id);
    println!("account id:  {}", account.account_id);
    println!("sequence:    {}", account.sequence);
}
