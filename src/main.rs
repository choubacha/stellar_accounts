#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io;

mod account;
mod asset;


fn main() {
    println!("Type 1 to get scotts testnet balance, Type 2 to get some MOBI info");
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Did not read input");
    let command = command.trim();
    if command == "1" {
        let account: account::Account = account::get();
        println!("id:          {}", account.id);
        println!("account id:  {}", account.account_id);
        println!("sequence:    {}", account.sequence);
    } else if command == "2" {
        let asset: asset::Asset = asset::get();
        println!("{} accounts hold {} issued by {}", asset.num_accounts, asset.asset_code, asset.asset_issuer);
    } else {
        println!("Don't be silly");
    }
}
