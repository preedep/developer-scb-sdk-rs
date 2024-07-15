use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillPaymentTransaction {
    // Transaction Slip ID
    #[serde(rename = "transRef")]
    pub trans_ref: String,
    // Sender Bank Code
    #[serde(rename = "sendingBank")]
    pub sending_bank: String,
    // Receiving Bank Code
    #[serde(rename = "receivingBank")]
    pub receiving_bank: String,
    // Transaction Date
    #[serde(rename = "transDate")]
    pub trans_date: String,
    // Transaction Time
    #[serde(rename = "transTime")]
    pub trans_time: String,
    // See Sender
    #[serde(rename = "sender")]
    pub sender: Sender,
    // See Receiver
    #[serde(rename = "receiver")]
    pub receiver: Receiver,
    // Transaction Amount
    #[serde(rename = "amount")]
    pub amount: String,
    // Local Amount
    #[serde(rename = "paidLocalAmount")]
    pub paid_local_amount: String,
    // Local Currency
    #[serde(rename = "paidLocalCurrency")]
    pub paid_local_currency: String,
    // Country Code
    #[serde(rename = "countryCode")]
    pub country_code: String,
    // Reference 1
    #[serde(rename = "ref1")]
    pub ref1: String,
    // Reference 2
    #[serde(rename = "ref2")]
    pub ref2: String,
    // Reference 3
    #[serde(rename = "ref3")]
    pub ref3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    // Display name for payer
    #[serde(rename = "displayName")]
    pub display_name: String,
    // Name of payer
    #[serde(rename = "name")]
    pub name: String,
    // See Proxy
    #[serde(rename = "proxy")]
    pub proxy: Proxy,
    // See Account
    #[serde(rename = "account")]
    pub account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receiver {
    // Display name for payee
    #[serde(rename = "displayName")]
    pub display_name: String,
    // Name of payee
    #[serde(rename = "name")]
    pub name: String,
    // See Proxy
    #[serde(rename = "proxy")]
    pub proxy: Proxy,
    // See Account
    #[serde(rename = "account")]
    pub account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    // BILLERID
    #[serde(rename = "type")]
    pub type_field: String,
    // BILLER ID
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    // BANKAC
    #[serde(rename = "type")]
    pub type_field: String,
    // Account Number
    #[serde(rename = "value")]
    pub value: String,
}

