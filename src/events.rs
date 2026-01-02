use odra::prelude::*;
use odra::casper_types::U256;


#[odra::event]
pub struct MerchantRegistered {
    pub merchant_id: String, 
    pub wallet_address: Address,
    pub timestamp: u64,
}

#[odra::event]
pub struct MerchantStatusChanged {
    pub merchant_id: String,
    pub old_status: u8,
    pub new_status: u8,
    pub timestamp: u64,
}

#[odra::event]
pub struct ProductCreated {
    pub merchant_id: String, 
    pub product_id: String,
    pub price: U256,
    pub timestamp: u64,
}

#[odra::event]
pub struct ProductUpdated {
    pub merchant_id: String,
    pub product_id: String,
    pub timestamp: u64,
}

#[odra::event]
pub struct SubscriptionPlanCreated {
    pub merchant_id: String, 
    pub plan_id: String,
    pub price: U256,
    pub interval: u8, // 0=weekly, 1=monthly, 2=yearly
    pub trial_days: u32,
    pub timestamp: u64,
}

#[odra::event]
pub struct SubscriptionPlanUpdated {
    pub merchant_id: String,
    pub plan_id: String,
    pub timestamp: u64,
}

#[odra::event]
pub struct PaymentReceived {
    pub payer: Address,
    pub product_id: Option<String>,
    pub subscription_plan_id: Option<String>,
    pub timestamp: u64,
}

#[odra::event]
pub struct SubscriptionStarted {
    pub merchant_id: String, 
    pub subscriber: Address,
    pub plan_id: String, 
    pub subscription_id: String, 
    pub period_start: u64,
    pub period_end: u64,
    pub trial_end: Option<u64>,
    pub timestamp: u64,
}

#[odra::event]
pub struct SubscriptionCharged {
    pub merchant_id: String,
    pub subscriber: Address,
    pub plan_id: String,
    pub amount: U256,
    pub token_address: String,
    pub period_start: u64,
    pub period_end: u64,
    pub success: bool,
    pub timestamp: u64,
}

#[odra::event]
pub struct SubscriptionStatusChanged {
    pub merchant_id: String,
    pub subscriber: Address,
    pub plan_id: String,
    pub old_status: u8,
    pub new_status: u8,
    pub timestamp: u64,
}

#[odra::event]
pub struct Withdrawal {
    pub merchant_id: String,
    pub recipient: Address,
    pub amount: U256,
    pub token_address: String,
    pub timestamp: u64,
}

#[odra::event]
pub struct ConfigurationUpdated {
    pub admin: Address,
    pub config_type: String,
    pub timestamp: u64,
}
