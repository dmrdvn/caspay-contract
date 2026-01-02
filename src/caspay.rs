use odra::prelude::*;
use odra::module::SubModule;
use odra::casper_types::U256;
use crate::{
    errors::CasPayError,
    events::*,
    storage::CasPayStorage,
    types::*,
};

/// CasPay Core Contract
/// Single contract with dictionary-based multi-tenancy for scalable payment gateway
#[odra::module(events = [
    MerchantRegistered,
    MerchantStatusChanged,
    ProductCreated,
    ProductUpdated,
    SubscriptionPlanCreated,
    SubscriptionPlanUpdated,
    PaymentReceived,
    SubscriptionStarted,
    SubscriptionCharged,
    SubscriptionStatusChanged,
    Withdrawal,
    ConfigurationUpdated,
])]
pub struct CasPay {
    storage: SubModule<CasPayStorage>,
}

#[odra::module]
impl CasPay {
    /// Initialize contract
    fn ensure_initialized(&mut self, admin: Address) {
        let existing_config = self.storage.platform_config.get();
        if existing_config.is_none() {
            let config = PlatformConfig {
                admin,
                paused: false,
            };
            self.storage.platform_config.set(config);
            self.storage.init();
            
            self.env().emit_event(ConfigurationUpdated {
                admin,
                config_type: String::from("initial_setup"),
                timestamp: self.env().get_block_time(),
            });
        }
    }

    pub fn init(
        &mut self,
        admin: Address,
    ) {
        self.ensure_initialized(admin);
    }

    // ==================== MERCHANT MANAGEMENT ====================

    pub fn register_merchant(
        &mut self,
        merchant_id: String, 
        wallet_address: Address,
    ) {
        let caller = self.env().caller();
        self.ensure_initialized(caller);
        
        self.require_not_paused();
        self.require_admin();

        // Check merchant doesn't already exist
        if self.storage.merchant_exists(&merchant_id) {
            self.env().revert(CasPayError::MerchantAlreadyExists);
        }

        let timestamp = self.env().get_block_time();

        let config = MerchantConfig {
            merchant_id: merchant_id.clone(),
            wallet_address,
            status: MerchantStatus::Active,
            created_at: timestamp,
            updated_at: timestamp,
            active: true,
        };

        self.storage.set_merchant(merchant_id.clone(), config);

        self.env().emit_event(MerchantRegistered {
            merchant_id,
            wallet_address,
            timestamp,
        });
    }

    /// Update merchant status (admin only)
    pub fn update_merchant_status(
        &mut self,
        merchant_id: String,
        new_status: MerchantStatus,
    ) {
        self.require_admin();

        let mut merchant = self.storage
            .get_merchant(&merchant_id)
            .unwrap_or_revert_with(&self.env(), CasPayError::InvalidMerchant);

        let old_status = merchant.status;
        merchant.status = new_status;
        merchant.updated_at = self.env().get_block_time();

        self.storage.set_merchant(merchant_id.clone(), merchant);

        self.env().emit_event(MerchantStatusChanged {
            merchant_id,
            old_status: old_status as u8,
            new_status: new_status as u8,
            timestamp: self.env().get_block_time(),
        });
    }

    /// Get merchant configuration (read-only)
    pub fn get_merchant(&self, merchant_id: String) -> Option<MerchantConfig> {
        self.storage.get_merchant(&merchant_id)
    }

    // ==================== PRODUCT MANAGEMENT ====================

    pub fn create_product(
        &mut self,
        merchant_id: String, 
        product_id: String,
        price: U256,
    ) {
        self.require_not_paused();
        self.require_admin();

        // Validate merchant exists
        if !self.storage.merchant_exists(&merchant_id) {
            self.env().revert(CasPayError::InvalidMerchant);
        }

        // Validate amount
        if price == U256::zero() {
            self.env().revert(CasPayError::InvalidAmount);
        }

        // Use product_id as key (no concatenation needed)
        if self.storage.product_exists(&product_id) {
            self.env().revert(CasPayError::ProductAlreadyExists);
        }

        let timestamp = self.env().get_block_time();

        let product = Product {
            product_id: product_id.clone(),
            merchant_id: merchant_id.clone(),
            price,
            active: true,
            created_at: timestamp,
            updated_at: timestamp,
        };

        self.storage.set_product(product_id.clone(), product);

        self.env().emit_event(ProductCreated {
            merchant_id,
            product_id,
            price,
            timestamp,
        });
    }

    /// Update product active status (admin only)
    pub fn update_product_status(
        &mut self,
        product_id: String,
        active: bool,
    ) {
        self.require_admin();
        
        let mut product = self.storage
            .get_product(&product_id)
            .unwrap_or_revert_with(&self.env(), CasPayError::InvalidProduct);

        product.active = active;
        product.updated_at = self.env().get_block_time();
        self.storage.set_product(product_id.clone(), product.clone());

        self.env().emit_event(ProductUpdated {
            merchant_id: product.merchant_id,
            product_id,
            timestamp: self.env().get_block_time(),
        });
    }

    /// Get product information (read-only)
    pub fn get_product(&self, product_id: String) -> Option<Product> {
        self.storage.get_product(&product_id)
    }

    // ==================== SUBSCRIPTION PLAN MANAGEMENT ====================

    pub fn create_subscription_plan(
        &mut self,
        merchant_id: String, 
        plan_id: String,
        price: U256,
        interval: SubscriptionInterval,
        interval_count: u32,
        trial_days: u32,
    ) {
        self.require_not_paused();
        self.require_admin();

        // Validate merchant exists
        if !self.storage.merchant_exists(&merchant_id) {
            self.env().revert(CasPayError::InvalidMerchant);
        }

        // Validate amount
        if price == U256::zero() {
            self.env().revert(CasPayError::InvalidAmount);
        }

        if self.storage.plan_exists(&plan_id) {
            self.env().revert(CasPayError::SubscriptionPlanAlreadyExists);
        }

        let timestamp = self.env().get_block_time();

        let plan = SubscriptionPlan {
            plan_id: plan_id.clone(),
            merchant_id: merchant_id.clone(),
            price,
            interval,
            interval_count,
            trial_days,
            active: true,
            created_at: timestamp,
            updated_at: timestamp,
        };

        self.storage.set_plan(plan_id.clone(), plan);

        self.env().emit_event(SubscriptionPlanCreated {
            merchant_id,
            plan_id,
            price,
            interval: interval as u8,
            trial_days,
            timestamp,
        });
    }

    /// Update subscription plan status (admin only)
    pub fn update_plan_status(
        &mut self,
        plan_id: String, 
        active: bool,
    ) {
        self.require_admin();
        
        let mut plan = self.storage
            .get_plan(&plan_id)
            .unwrap_or_revert_with(&self.env(), CasPayError::InvalidSubscriptionPlan);

        plan.active = active;
        plan.updated_at = self.env().get_block_time();
        self.storage.set_plan(plan_id.clone(), plan.clone());

        self.env().emit_event(SubscriptionPlanUpdated {
            merchant_id: plan.merchant_id,
            plan_id,
            timestamp: self.env().get_block_time(),
        });
    }

    /// Get subscription plan (read-only)
    pub fn get_subscription_plan(&self, plan_id: String) -> Option<SubscriptionPlan> {
        self.storage.get_plan(&plan_id)
    }

    // ==================== PAYMENT PROCESSING ====================

    /// Record a payment (admin only)
    /// Called from backend API after transaction verification
    /// Only minimal data stored on-chain
    pub fn record_payment(
        &mut self,
        payer: Address,
        product_id: Option<String>,
        subscription_plan_id: Option<String>,
    ) {
        self.require_not_paused();
        self.require_admin();

        // Store payment record (minimal data only)
        let timestamp = self.env().get_block_time();
        
        // Generate unique payment key from payer + timestamp
        let payment_key = format!("{:?}:{}", payer, timestamp);
        
        let payment = PaymentRecord {
            payer,
            product_id: product_id.clone(),
            subscription_plan_id: subscription_plan_id.clone(),
            timestamp,
        };

        self.storage.set_payment(payment_key, payment);

        self.env().emit_event(PaymentReceived {
            payer,
            product_id,
            subscription_plan_id,
            timestamp,
        });
    }

    /// Get payment record by key (read-only)
    pub fn get_payment(&self, payment_key: String) -> Option<PaymentRecord> {
        self.storage.get_payment(&payment_key)
    }

    // ==================== SUBSCRIPTION MANAGEMENT ====================

    pub fn create_subscription(
        &mut self,
        subscription_id: String,    
        merchant_id: String, 
        subscriber: Address,
        plan_id: String, 
    ) {
        self.require_not_paused();
        self.require_admin();

        // Validate plan exists
        let plan = self.storage
            .get_plan(&plan_id)
            .unwrap_or_revert_with(&self.env(), CasPayError::InvalidSubscriptionPlan);

        if !plan.active {
            self.env().revert(CasPayError::InvalidSubscriptionPlan);
        }

        // Check subscription doesn't already exist
        if self.storage.subscription_exists(&subscription_id) {
            self.env().revert(CasPayError::SubscriptionAlreadyActive);
        }

        let timestamp = self.env().get_block_time();
        
        // Calculate periods
        let trial_start = if plan.trial_days > 0 {
            Some(timestamp)
        } else {
            None
        };

        let trial_end = if plan.trial_days > 0 {
            Some(timestamp + (plan.trial_days as u64 * 86400))
        } else {
            None
        };

        let period_start = timestamp;
        let period_end = period_start + (30 * 86400);

        let next_charge_date = if trial_end.is_some() {
            trial_end
        } else {
            Some(period_end)
        };

        let status = if plan.trial_days > 0 {
            SubscriptionStatus::Trialing
        } else {
            SubscriptionStatus::Active
        };

        let subscription = Subscription {
            merchant_id: merchant_id.clone(),
            subscriber,
            plan_id: plan_id.clone(),
            status,
            current_period_start: period_start,
            current_period_end: period_end,
            next_charge_date,
            trial_start,
            trial_end,
            cancel_at_period_end: false,
            cancelled_at: None,
            created_at: timestamp,
            updated_at: timestamp,
        };

        self.storage.set_subscription(subscription_id.clone(), subscription);

        self.env().emit_event(SubscriptionStarted {
            merchant_id,
            subscriber,
            plan_id,
            subscription_id,
            period_start,
            period_end,
            trial_end,
            timestamp,
        });
    }

    /// Check if subscription is active (read-only - for dApp access control)
    pub fn check_subscription(&self, subscription_id: String) -> bool {
        if let Some(subscription) = self.storage.get_subscription(&subscription_id) {
            let current_time = self.env().get_block_time();
            
            // Check if subscription is active and not expired
            if subscription.status == SubscriptionStatus::Active ||
               subscription.status == SubscriptionStatus::Trialing {
                return subscription.current_period_end >= current_time;
            }
        }

        false
    }

    /// Get subscription details (read-only)
    pub fn get_subscription(&self, subscription_id: String) -> Option<Subscription> {
        self.storage.get_subscription(&subscription_id)
    }

    /// Cancel subscription (admin only or subscriber)
    pub fn cancel_subscription(
        &mut self,
        subscription_id: String, // Supabase UUID
        cancel_at_period_end: bool,
    ) {
        self.require_not_paused();

        let mut subscription = self.storage
            .get_subscription(&subscription_id)
            .unwrap_or_revert_with(&self.env(), CasPayError::InvalidSubscription);

        // Check if caller is admin or subscriber
        let caller = self.env().caller();
        let config = self.get_platform_config();
        if caller != config.admin && caller != subscription.subscriber {
            self.env().revert(CasPayError::Unauthorized);
        }

        let old_status = subscription.status;
        let timestamp = self.env().get_block_time();

        if cancel_at_period_end {
            subscription.cancel_at_period_end = true;
            subscription.cancelled_at = Some(timestamp);
        } else {
            subscription.status = SubscriptionStatus::Cancelled;
            subscription.cancelled_at = Some(timestamp);
        }

        subscription.updated_at = timestamp;
    
        self.storage.set_subscription(subscription_id.clone(), subscription.clone());
    
        self.env().emit_event(SubscriptionStatusChanged {
            merchant_id: subscription.merchant_id,
            subscriber: subscription.subscriber,
            plan_id: subscription.plan_id,
            old_status: old_status as u8,
            new_status: subscription.status as u8,
            timestamp,
        });
    }

    // ==================== BALANCE & WITHDRAWAL ====================

    /// Get merchant balance for a specific token (read-only)
    pub fn get_merchant_balance(
        &self,
        merchant_id: String,
        token_address: String,
    ) -> MerchantBalance {
        let balance_key = format!("{}:{}", merchant_id, token_address);
        self.storage.get_merchant_balance(&balance_key)
    }

    /// Withdraw funds (admin only)
    /// Called from backend API when merchant requests withdrawal
    pub fn withdraw(
        &mut self,
        merchant_id: String,
        token_address: String,
        amount: U256,
        recipient: Address,
    ) {
        self.require_not_paused();
        self.require_admin();

        // Validate amount
        if amount == U256::zero() {
            self.env().revert(CasPayError::InvalidAmount);
        }

        let balance_key = format!("{}:{}", merchant_id, token_address);
        let mut balance = self.storage.get_merchant_balance(&balance_key);

        // Check sufficient balance
        if balance.available < amount {
            self.env().revert(CasPayError::InsufficientBalance);
        }

        // Update balance
        balance.available = balance.available - amount;
        balance.total_withdrawn = balance.total_withdrawn + amount;
        
        self.storage.set_merchant_balance(balance_key, balance);

        self.env().emit_event(Withdrawal {
            merchant_id,
            recipient,
            amount,
            token_address,
            timestamp: self.env().get_block_time(),
        });
    }

    // ==================== ADMIN FUNCTIONS ====================

    /// Pause contract (admin only - emergency stop)
    pub fn pause_contract(&mut self) {
        self.require_admin();

        let mut config = self.get_platform_config();
        config.paused = true;
        self.storage.platform_config.set(config);
    }

    /// Unpause contract (admin only)
    pub fn unpause_contract(&mut self) {
        self.require_admin();

        let mut config = self.get_platform_config();
        config.paused = false;
        self.storage.platform_config.set(config);
    }

    /// Get platform revenue statistics (read-only)
    pub fn get_platform_revenue(&self) -> U256 {
        self.storage.get_platform_revenue()
    }

    // ==================== HELPER FUNCTIONS ====================

    fn require_admin(&self) {
        let config = self.get_platform_config();
        let caller = self.env().caller();
        
        if caller != config.admin {
            self.env().revert(CasPayError::Unauthorized);
        }
    }

    fn require_not_paused(&self) {
        let config = self.get_platform_config();
        if config.paused {
            self.env().revert(CasPayError::ContractPaused);
        }
    }

    fn get_platform_config(&self) -> PlatformConfig {
        self.storage
            .platform_config
            .get()
            .unwrap_or_revert_with(&self.env(), CasPayError::Unauthorized)
    }
}
