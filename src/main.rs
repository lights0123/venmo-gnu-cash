use std::io;

use serde::{Deserialize, Serialize};

use crate::parser::Money;

mod parser;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Transaction {
    #[serde(rename = "ID")]
    id: u64,
    datetime: chrono::naive::NaiveDateTime,
    r#type: String,
    note: String,
    from: String,
    to: String,
    #[serde(rename = "Amount (total)")]
    amount: Money,
    #[serde(rename = "Funding Source")]
    funding_source: String,
    destination: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct GnuCashTransaction<'a> {
    num: Option<u64>,
    date: chrono::naive::NaiveDate,
    description: &'a str,
    account: &'a str,
    amount: String,
    notes: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for result in rdr.deserialize() {
        let record: Transaction = match result {
	        Ok(v) => v,
	        Err(_) => continue,
        };
        match record.funding_source.as_ref() {
            "Venmo balance" | "" => {}
            account => {
                wtr.serialize(GnuCashTransaction {
                    num: None,
                    date: record.datetime.date(),
                    description: "VENMO PAYMENT",
                    account,
                    amount: record.amount.abs().to_string(),
                    notes: "",
                })?;
            }
        }
        let account = match record.destination.as_ref() {
            "Venmo balance" | "" => "Other",
            account => account,
        };
        let destination = if (record.amount.0 >= 0) == (record.r#type == "Payment") {
            &record.from
        } else {
            &record.to
        };
        let description =
            if record.destination == "Venmo balance" || record.funding_source == "Venmo balance" {
                &record.note
            } else {
                "VENMO CASHOUT"
            };
        wtr.serialize(GnuCashTransaction {
            num: Some(record.id),
            date: record.datetime.date(),
            description,
            account,
            amount: record.amount.to_string(),
            notes: destination,
        })?;
        wtr.flush()?;
    }
    Ok(())
}
