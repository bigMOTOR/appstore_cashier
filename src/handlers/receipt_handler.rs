use serde::{Deserialize, Serialize};

const VERIFY_RECEIPT_SANDBOX: &str = "https://sandbox.itunes.apple.com/verifyReceipt";
const VERIFY_RECEIPT_PROD: &str = "https://buy.itunes.apple.com/verifyReceipt";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReceiptPayload {
    /// The Base64-encoded receipt data. (Required)
    pub receipt_data: String,
    /// Your app’s shared secret, which is a hexadecimal string. (Required)
    pub password: String,
    /// Set this value to true for the response to include only the latest renewal transaction for any subscriptions. Use this field only for app receipts that contain auto-renewable subscriptions.
    pub exclude_old_transactions: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct VerifyDto {
    // https://developer.apple.com/documentation/appstorereceipts/responsebody
    environment: Environment,
    is_retryable: bool,
    latest_receipt: String,

}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Environment {
    Sandbox,
    Production
}