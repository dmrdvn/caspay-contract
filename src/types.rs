use odra::prelude::*;
use odra::casper_types::U256;


#[odra::odra_type]
#[derive(Copy)]
pub enum MerchantStatus {
    Pending = 0,
    Active = 1,
    Suspended = 2,
    Closed = 3,
}

#[odra::odra_type]
pub struct MerchantConfig {
    pub merchant_id: String,
    pub wallet_address: Address, 
    pub status: MerchantStatus,
    pub created_at: u64,
    pub updated_at: u64, 
    pub active: bool,
}

#[odra::odra_type]
pub struct Product {
    pub product_id: String, 
    pub merchant_id: String, 
    pub price: U256,
    pub active: bool,
    pub created_at: u64,
    pub updated_at: u64, 
}

#[odra::odra_type]
#[derive(Copy)]
pub enum SubscriptionInterval {
    Day = 0,
    Week = 1,
    Month = 2,
    Year = 3,
}

#[odra::odra_type]
pub struct SubscriptionPlan {
    pub plan_id: String, 
    pub merchant_id: String, 
    pub price: U256,
    pub interval: SubscriptionInterval,
    pub interval_count: u32,
    pub trial_days: u32,
    pub active: bool,
    pub created_at: u64,
    pub updated_at: u64, 
}

#[odra::odra_type]
#[derive(Copy)]
pub enum SubscriptionStatus {
    Active = 0,
    Trialing = 1,
    Paused = 2,
    Cancelled = 3,
    PastDue = 4,
    Expired = 5,
}

#[odra::odra_type]
pub struct Subscription {
    pub merchant_id: String,
    pub subscriber: Address,
    pub plan_id: String,
    pub status: SubscriptionStatus,
    pub current_period_start: u64,
    pub current_period_end: u64,
    pub next_charge_date: Option<u64>,
    pub trial_start: Option<u64>,
    pub trial_end: Option<u64>,
    pub cancel_at_period_end: bool,
    pub cancelled_at: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[odra::odra_type]
pub struct PaymentRecord {
    pub payer: Address,
    pub product_id: Option<String>,
    pub subscription_plan_id: Option<String>,
    pub timestamp: u64,
}

#[odra::odra_type]
pub struct MerchantBalance {
    pub available: U256,
    pub pending: U256,
    pub total_received: U256,
    pub total_withdrawn: U256,
}

impl Default for MerchantBalance {
    fn default() -> Self {
        Self {
            available: U256::zero(),
            pending: U256::zero(),
            total_received: U256::zero(),
            total_withdrawn: U256::zero(),
        }
    }
}

#[odra::odra_type]
pub struct PlatformConfig {
    pub admin: Address,
    pub paused: bool,
}
