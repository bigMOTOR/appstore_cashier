use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReceiptPayload {
    /// The Base64-encoded receipt data. (Required)
    pub receipt_data: String,
    /// Your app’s shared secret, which is a hexadecimal string. (Required)
    pub password: String,
    /// Set this value to true for the response to include only the latest renewal transaction for any subscriptions. Use this field only for app receipts that contain auto-renewable subscriptions.
    pub exclude_old_transactions: bool
}
