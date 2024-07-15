use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::validate::validate_data_type_date;

#[derive(Debug, Clone, Serialize, Deserialize,Validate)]
pub struct BillPaymentInquiryRequest {
    // Event code of payment type
    // Possible value:
    // 00300100 - Thai QR Code Tag 30 (C Scan B)
    // 00300104 - My Prompt QR (B Scan C)
    #[validate(length(min = 1, max = 10))]
    #[serde(rename = "eventCode")]
    pub event_code: String,

    // Date of transaction.
    // Format: yyyy-MM-dd
    // Example: 2019-10-28
    #[validate(length(min = 10, max = 10),custom(function = "validate_data_type_date"))]
    #[serde(rename = "transactionDate")]
    pub transaction_date: String,

    // Biller ID from partner
    // Required if: eventCode = 00300100
    #[validate(length(max = 15))]
    #[serde(rename = "billerId",skip_serializing_if = "Option::is_none")]
    pub biller_id: Option<String>,

    // Reference Number 1 , up to 20 characters
    // Required if: eventCode = 00300100
    #[validate(length(max = 20))]
    #[serde(rename = "reference1",skip_serializing_if = "Option::is_none")]
    pub reference1: Option<String>,

    // Reference Number 2 , up to 20 characters
    #[validate(length(max = 20))]
    #[serde(rename = "reference2",skip_serializing_if = "Option::is_none")]
    pub reference2: Option<String>,

    // Transaction ID from a partner
    // Required if: eventCode = 00300104
    #[validate(length(max = 35))]
    #[serde(rename = "partnerTransactionId",skip_serializing_if = "Option::is_none")]
    pub partner_transaction_id: Option<String>,

    // Transaction Amount
    #[serde(rename = "amount",skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  BillPaymentTransaction{

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillPaymentTransactionSlip {
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

