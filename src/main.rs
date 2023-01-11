use clap::Parser;
use csv::Writer;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

/// A CLI to generate DAO drops from Cosmos chain JSON exports
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the chain export JSON file
    #[arg(short, long)]
    file_path: String,
    /// Minimum staked amount (in ujuno) to be eligible
    #[arg(short, long, default_value_t = 1000000)]
    min_staked_amount: u128,
    /// Cap the maximum amount (in ujuno) an account is eligible for
    #[arg(short, long, default_value_t = 100000000000000000)]
    whale_cap: u128,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data = read_file(&args.file_path);

    // Parse JSON into serde value
    let v: Value = serde_json::from_str(data.as_str())?;

    // Initialize CSV writer
    let mut wtr = Writer::from_path("drop.csv")?;

    // Format used by https://juno.tools
    wtr.write_record(&["address", "amount"])?;

    // Struct to hold account info
    pub struct Account<'a> {
        pub address: &'a str,
        pub amount: u128,
    }

    // A hash map containing accounts receiving the drop
    let mut drop: HashMap<&str, Account> = HashMap::new();

    for i in v["app_state"]["staking"]["delegations"].as_array().unwrap() {
        let address = i["delegator_address"].as_str().unwrap();
        let amount = i["shares"]
            .as_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .parse::<u128>()?;

        // Update hash map of accounts, increment existing entries for accounts that stake to many validators
        drop.entry(address)
            .and_modify(|acc| {
                acc.amount += amount;
                // If new amount is over whale cap, set amount to whale cap
                if acc.amount > args.whale_cap {
                    acc.amount = args.whale_cap;
                }
            })
            .or_insert(Account { address, amount });
    }

    let mut total_drop_amount = 0;
    let mut total_drop_count = 0;

    // Iterate through hashmap to create CSV and totals
    for (_, account) in drop.into_iter() {
        if account.amount > args.min_staked_amount {
            wtr.write_record(&[account.address, &account.amount.to_string()])?;
            total_drop_amount += account.amount;
            total_drop_count += 1;
        }
    }

    println!("Number of accounts: {:?}", total_drop_count);
    println!("Total drop amount: {:?}", total_drop_amount);

    // Write CSV
    wtr.flush()?;

    Ok(())
}

// Reads a large JSON file as a buffer
// Credit: https://goksh.com/posts/2020/06/parsing-large-json-in-rust-as-untyped-or-strongly-typed-data/
pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("could not read file into the string");
    contents
}
