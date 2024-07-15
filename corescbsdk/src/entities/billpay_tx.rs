use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillPaymentTransaction {
    pub trans_ref: String,
    pub sending_bank: String,
    pub receiving_bank: String,
    pub trans_date: String,
    pub trans_time: String,
    pub sender: Sender,
    pub receiver: Receiver,
    pub amount: String,
    pub paid_local_amount: String,
    pub paid_local_currency: String,
    pub country_code: String,
    pub ref1: String,
    pub ref2: String,
    pub ref3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    pub display_name: String,
    pub name: String,
    pub proxy: Proxy,
    pub account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receiver {
    pub display_name: String,
    pub name: String,
    pub proxy: Proxy,
    pub account: Account,
}