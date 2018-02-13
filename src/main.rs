#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    account_id: String,
    sequence: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    asset_code: String,
    asset_issuer: String,
    num_accounts: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embedded<T> {
    _embedded: Records<T>
}

#[derive(Serialize, Deserialize, Debug)]
struct Records<T> {
    records: Vec<T>
}

fn main() {
    let client = reqwest::Client::new();

    println!("Type 1 to get scotts testnet balance, Type 2 to get some MOBI info");
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Did not read input");
    let command = command.trim();
    if command == "1" {
        let url = "https://horizon-testnet.stellar.org";
        let account_id = "GATUX2IIOOPQ5CWW3SFVOFUKBBSJ72GIGROR3U4PAA7VUYZT5AYD3HL4";
        let mut response = client.get(&format!("{}/accounts/{}", url, account_id))
            .send()
            .unwrap();
        let json_str = response.text().expect("there was no body");
        let account: Account = serde_json::from_str::<Account>(&json_str)
            .expect("invalid json");
        println!("The id is: {}", account.id);
    } else if command == "2" {
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
        let asset = &assets[0];
        println!("{} accounts hold {} issued by {}", asset.num_accounts, asset.asset_code, asset.asset_issuer);
    } else {
        println!("Don't be silly");
    }
}