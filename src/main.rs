#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};
use serde::de;

fn deserialize_from_str<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
{
    // Call string deserialize on the "deserializer".
    let s = String::deserialize(d)?;
    // Now that we have a string, we can call FromStr
    T::from_str(&s).map_err(|_| de::Error::custom("failed to parse string field"))
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    account_id: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    sequence: u64,
    subentry_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    asset_code: String,
    asset_issuer: String,
    num_accounts: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Embedded<T> {
    _embedded: Records<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Records<T> {
    records: Vec<T>,
}

enum Command {
    ScottsBalance,
    MobiInfo,
    TryAgain,
    Quit,
}

#[cfg(test)]
mod resource_tests {
    use super::*;

    fn account_json() -> &'static str {
        include_str!("example_account.json")
    }

    #[test]
    fn it_can_serialize_an_account() {
        let account: Account = serde_json::from_str(&account_json()).unwrap();
        assert_eq!(account.sequence, 31429458275598336);
        assert_eq!(account.id, "GATUX2IIOOPQ5CWW3SFVOFUKBBSJ72GIGROR3U4PAA7VUYZT5AYD3HL4");
        assert_eq!(account.subentry_count, 0);
    }
}

fn get_command() -> Command {
    println!("Type 1 to get scotts testnet balance, Type 2 to get some MOBI info\n");

    let mut command = String::new();
    match io::stdin().read_line(&mut command) {
        Ok(n) if n > 1 => match command.trim() {
            "1" => Command::ScottsBalance,
            "2" => Command::MobiInfo,
            "q" => Command::Quit,
            _ => Command::TryAgain,
        },
        Ok(_) => Command::Quit,
        Err(e) => {
            println!("Something went wrong: {}", e);
            Command::TryAgain
        }
    }
}

fn main() {
    let client = reqwest::Client::new();
    loop {
        match get_command() {
            Command::ScottsBalance => {
                let url = "https://horizon-testnet.stellar.org";
                let account_id = "GATUX2IIOOPQ5CWW3SFVOFUKBBSJ72GIGROR3U4PAA7VUYZT5AYD3HL4";
                let mut response = client
                    .get(&format!("{}/accounts/{}", url, account_id))
                    .send()
                    .unwrap();
                let json_str = response.text().expect("there was no body");
                let account: Account =
                    serde_json::from_str::<Account>(&json_str).expect("invalid json");
                println!("id:          {}", account.id);
                println!("account id:  {}", account.account_id);
                println!("sequence:    {}", account.sequence);
            }
            Command::MobiInfo => {
                let url = "https://horizon.stellar.org";
                let asset_code = "MOBI";
                let asset_issuer = "GBNMSB7UXFOGWC3E6BQRKYOKJCAYUL6WBZMCINHKGWAD3PTRXNWUQ2BB";
                let mut response = client
                    .get(&format!("{}/assets", url))
                    .query(&[("asset_code", asset_code), ("asset_issuer", asset_issuer)])
                    .send()
                    .unwrap();
                let json_str = response.text().expect("there was no body");
                let assets: Vec<Asset> = serde_json::from_str::<Embedded<Asset>>(&json_str)
                    .expect("invalid json")
                    ._embedded
                    .records;
                let asset = &assets[0];
                println!(
                    "{} accounts hold {} issued by {}",
                    asset.num_accounts, asset.asset_code, asset.asset_issuer
                );
            }
            Command::Quit => break,
            _ => continue,
        }
    }
}
