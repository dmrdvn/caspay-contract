use odra::prelude::*;
use odra::casper_types::U256;
use crate::types::*;

/// Storage module for CasPay contract

#[odra::module]
pub struct CasPayStorage {
    // Platform configuration
    pub platform_config: Var<PlatformConfig>,
    
    // Merchant data - Dictionary: merchant_id => MerchantConfig
    pub merchants: Mapping<String, MerchantConfig>,
    
    // Products - Dictionary: "merchant_id:product_id" => Product
    pub products: Mapping<String, Product>,
    
    // Subscription plans - Dictionary: "merchant_id:plan_id" => SubscriptionPlan
    pub subscription_plans: Mapping<String, SubscriptionPlan>,
    
    // Active subscriptions - Dictionary: "merchant_id:subscriber:plan_id" => Subscription
    pub subscriptions: Mapping<String, Subscription>,
    
    // Payment records - Dictionary: payment_key (payer:timestamp) => PaymentRecord
    pub payments: Mapping<String, PaymentRecord>,
    
    // Merchant balances - Dictionary: "merchant_id:token_address" => MerchantBalance
    // Token address is required to track different tokens (CSPR, USDT, USDC, etc.)
    pub merchant_balances: Mapping<String, MerchantBalance>,
    
    // Total platform revenue collected
    pub total_platform_revenue: Var<U256>,
}

#[odra::module]
impl CasPayStorage {
    /// Initialize storage with default values
    pub fn init(&mut self) {
        self.total_platform_revenue.set(U256::zero());
    }
    
    /// Check if merchant exists
    pub fn merchant_exists(&self, merchant_id: &String) -> bool {
        self.merchants.get(merchant_id).is_some()
    }
    
    /// Get merchant config
    pub fn get_merchant(&self, merchant_id: &String) -> Option<MerchantConfig> {
        self.merchants.get(merchant_id)
    }
    
    /// Set merchant config
    pub fn set_merchant(&mut self, merchant_id: String, config: MerchantConfig) {
        self.merchants.set(&merchant_id, config);
    }
    
    /// Check if product exists
    pub fn product_exists(&self, product_key: &String) -> bool {
        self.products.get(product_key).is_some()
    }
    
    /// Get product
    pub fn get_product(&self, product_key: &String) -> Option<Product> {
        self.products.get(product_key)
    }
    
    /// Set product
    pub fn set_product(&mut self, product_key: String, product: Product) {
        self.products.set(&product_key, product);
    }
    
    /// Check if subscription plan exists
    pub fn plan_exists(&self, plan_key: &String) -> bool {
        self.subscription_plans.get(plan_key).is_some()
    }
    
    /// Get subscription plan
    pub fn get_plan(&self, plan_key: &String) -> Option<SubscriptionPlan> {
        self.subscription_plans.get(plan_key)
    }
    
    /// Set subscription plan
    pub fn set_plan(&mut self, plan_key: String, plan: SubscriptionPlan) {
        self.subscription_plans.set(&plan_key, plan);
    }
    
    /// Check if subscription exists
    pub fn subscription_exists(&self, subscription_key: &String) -> bool {
        self.subscriptions.get(subscription_key).is_some()
    }
    
    /// Get subscription
    pub fn get_subscription(&self, subscription_key: &String) -> Option<Subscription> {
        self.subscriptions.get(subscription_key)
    }
    
    /// Set subscription
    pub fn set_subscription(&mut self, subscription_key: String, subscription: Subscription) {
        self.subscriptions.set(&subscription_key, subscription);
    }
    
    /// Check if payment exists
    pub fn payment_exists(&self, payment_key: &String) -> bool {
        self.payments.get(payment_key).is_some()
    }
    
    /// Get payment record
    pub fn get_payment(&self, payment_key: &String) -> Option<PaymentRecord> {
        self.payments.get(payment_key)
    }
    
    /// Set payment record
    pub fn set_payment(&mut self, payment_key: String, payment: PaymentRecord) {
        self.payments.set(&payment_key, payment);
    }
    
    /// Get merchant balance for specific token
    pub fn get_merchant_balance(&self, balance_key: &String) -> MerchantBalance {
        self.merchant_balances
            .get(balance_key)
            .unwrap_or_default()
    }
    
    /// Set merchant balance
    pub fn set_merchant_balance(&mut self, balance_key: String, balance: MerchantBalance) {
        self.merchant_balances.set(&balance_key, balance);
    }
    
    /// Add to platform revenue
    pub fn add_platform_revenue(&mut self, amount: U256) {
        let current = self.total_platform_revenue.get().unwrap_or(U256::zero());
        self.total_platform_revenue.set(current + amount);
    }
    
    /// Get total platform revenue
    pub fn get_platform_revenue(&self) -> U256 {
        self.total_platform_revenue.get().unwrap_or(U256::zero())
    }
}
