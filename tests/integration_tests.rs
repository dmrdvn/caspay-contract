#[cfg(test)]
mod tests {
    use odra::host::{Deployer, HostEnv};
    use odra::casper_types::U256;
    use odra::prelude::*;
    use caspay::CasPay;
    use caspay::caspay::{CasPayInitArgs, CasPayHostRef};
    use caspay::types::*;

    fn setup() -> (HostEnv, CasPayHostRef, Address, Address) {
        let test_env: HostEnv = odra_test::env();
        let admin = test_env.get_account(0);
        let merchant_wallet = test_env.get_account(1);
        test_env.set_caller(admin);
        
        let init_args = CasPayInitArgs { admin };
        let contract = CasPay::deploy(&test_env, init_args);
        
        (test_env, contract, admin, merchant_wallet)
    }

    // ==================== INITIALIZATION TESTS ====================

    #[test]
    fn test_initialization() {
        let (_, contract, _, _) = setup();
        let merchant = contract.get_merchant(String::from("nonexistent"));
        assert!(merchant.is_none());
    }

    // ==================== MERCHANT TESTS ====================

    #[test]
    fn test_register_merchant() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_001");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let merchant = contract.get_merchant(merchant_id).expect("Merchant should exist");
        assert_eq!(merchant.wallet_address, merchant_wallet);
        assert_eq!(merchant.status, MerchantStatus::Active);
    }

    #[test]
    #[should_panic]
    fn test_duplicate_merchant_fails() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_dup");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        contract.register_merchant(merchant_id, merchant_wallet); // Should panic
    }

    #[test]
    fn test_update_merchant_status() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_status");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        contract.update_merchant_status(merchant_id.clone(), MerchantStatus::Suspended);
        
        let merchant = contract.get_merchant(merchant_id).unwrap();
        assert_eq!(merchant.status, MerchantStatus::Suspended);
    }

    // ==================== PRODUCT TESTS ====================

    #[test]
    fn test_create_product() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_prod");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let product_id = String::from("prod_001");
        let price = U256::from(1000u64);
        contract.create_product(merchant_id.clone(), product_id.clone(), price);
        
        let product = contract.get_product(product_id).expect("Product should exist");
        assert_eq!(product.price, price);
        assert!(product.active);
    }

    #[test]
    #[should_panic]
    fn test_create_product_zero_price_fails() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_zero");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        contract.create_product(merchant_id, String::from("prod_001"), U256::zero());
    }

    // ==================== SUBSCRIPTION PLAN TESTS ====================

    #[test]
    fn test_create_subscription_plan() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_plan");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_monthly");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(99u64), SubscriptionInterval::Monthly, 1, 7
        );
        
        let plan = contract.get_subscription_plan(plan_id).expect("Plan should exist");
        assert_eq!(plan.trial_days, 7);
        assert_eq!(plan.interval, SubscriptionInterval::Monthly);
    }

    // ==================== PAYMENT TESTS ====================

    #[test]
    fn test_record_payment() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let payer = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_pay");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let product_id = String::from("prod_001");
        contract.create_product(merchant_id, product_id.clone(), U256::from(5000u64));
        
        // Record payment with only 3 parameters
        contract.record_payment(
            payer,
            Some(product_id),
            None,
        );
        
        // Payment key is generated from payer + timestamp
        // We can't predict the exact key, but the function should not panic
    }

    #[test]
    fn test_duplicate_payment_allowed() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let payer = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_dup_pay");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let product_id = String::from("prod_001");
        contract.create_product(merchant_id, product_id.clone(), U256::from(1000u64));
        
        // Multiple payments from same payer are now allowed (different timestamps)
        contract.record_payment(payer, Some(product_id.clone()), None);
        // Small delay to ensure different timestamp
        contract.record_payment(payer, Some(product_id), None);
    }

    // ==================== SUBSCRIPTION TESTS ====================

    #[test]
    fn test_create_subscription() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let subscriber = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_sub");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_basic");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(50u64), SubscriptionInterval::Monthly, 1, 0
        );
        
        let subscription_id = String::from("sub_001");
        contract.create_subscription(
            subscription_id.clone(), merchant_id, subscriber, plan_id
        );
        
        let subscription = contract.get_subscription(subscription_id.clone()).unwrap();
        assert_eq!(subscription.status, SubscriptionStatus::Active);
        assert!(contract.check_subscription(subscription_id));
    }

    #[test]
    fn test_cancel_subscription() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let subscriber = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_cancel");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_cancel");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(50u64), SubscriptionInterval::Monthly, 1, 0
        );
        
        let subscription_id = String::from("sub_cancel");
        contract.create_subscription(
            subscription_id.clone(), merchant_id, subscriber, plan_id
        );
        
        contract.cancel_subscription(subscription_id.clone(), false);
        
        let subscription = contract.get_subscription(subscription_id.clone()).unwrap();
        assert_eq!(subscription.status, SubscriptionStatus::Cancelled);
        assert!(!contract.check_subscription(subscription_id));
    }

    // ==================== SUBSCRIPTION QUERY TESTS ====================

    #[test]
    fn test_subscription_status_query() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let subscriber = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_query");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_query");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(50u64), SubscriptionInterval::Monthly, 1, 0
        );
        
        let subscription_id = String::from("sub_query");
        contract.create_subscription(
            subscription_id.clone(), merchant_id, subscriber, plan_id
        );
        
        // Test active subscription
        assert!(contract.check_subscription(subscription_id.clone()));
        
        // Cancel and test inactive subscription
        contract.cancel_subscription(subscription_id.clone(), false);
        assert!(!contract.check_subscription(subscription_id.clone()));
        
        // Test nonexistent subscription
        assert!(!contract.check_subscription(String::from("nonexistent_sub")));
    }

    #[test]
    fn test_get_subscription_details() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let subscriber = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_details");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_details");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(99u64), SubscriptionInterval::Monthly, 1, 0
        );
        
        let subscription_id = String::from("sub_details");
        contract.create_subscription(
            subscription_id.clone(), merchant_id.clone(), subscriber, plan_id.clone()
        );
        
        // Query subscription details
        let subscription = contract.get_subscription(subscription_id.clone()).unwrap();
        assert_eq!(subscription.merchant_id, merchant_id);
        assert_eq!(subscription.subscriber, subscriber);
        assert_eq!(subscription.plan_id, plan_id);
        assert_eq!(subscription.status, SubscriptionStatus::Active);
    }

    // ==================== AUTHORIZATION TESTS ====================

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_update_merchant_status() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let non_admin = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_auth_test");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.update_merchant_status(merchant_id, MerchantStatus::Suspended);
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_create_product() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let non_admin = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_product_auth");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.create_product(
            merchant_id, String::from("prod_001"), U256::from(100u64)
        );
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_create_subscription_plan() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let non_admin = test_env.get_account(2);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_plan_auth");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.create_subscription_plan(
            merchant_id, String::from("plan_001"),
            U256::from(50u64), SubscriptionInterval::Monthly, 1, 0
        );
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_record_payment() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let non_admin = test_env.get_account(2);
        let payer = test_env.get_account(3);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_payment_auth");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let product_id = String::from("prod_001");
        contract.create_product(merchant_id, product_id.clone(), U256::from(1000u64));
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.record_payment(payer, Some(product_id), None);
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_withdraw() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let non_admin = test_env.get_account(2);
        let payer = test_env.get_account(3);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_withdraw_auth");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let product_id = String::from("prod_001");
        contract.create_product(merchant_id.clone(), product_id.clone(), U256::from(5000u64));
        
        // Record payment with only 3 parameters
        contract.record_payment(payer, Some(product_id), None);
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.withdraw(
            merchant_id, String::from("NATIVE"),
            U256::from(1000u64), merchant_wallet
        );
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_pause_contract() {
        let (test_env, mut contract, _, _) = setup();
        let non_admin = test_env.get_account(2);
        
        test_env.set_caller(non_admin);
        contract.pause_contract();
    }

    #[test]
    #[should_panic]
    fn test_non_admin_cannot_create_subscription() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        let subscriber = test_env.get_account(2);
        let non_admin = test_env.get_account(3);
        test_env.set_caller(admin);
        
        let merchant_id = String::from("merchant_sub_auth");
        contract.register_merchant(merchant_id.clone(), merchant_wallet);
        
        let plan_id = String::from("plan_auth");
        contract.create_subscription_plan(
            merchant_id.clone(), plan_id.clone(),
            U256::from(50u64), SubscriptionInterval::Monthly, 1, 0
        );
        
        // Switch to non-admin
        test_env.set_caller(non_admin);
        contract.create_subscription(
            String::from("sub_auth"), merchant_id, subscriber, plan_id
        );
    }

    // ==================== ADMIN CONTROL TESTS ====================

    #[test]
    fn test_pause_unpause() {
        let (test_env, mut contract, admin, merchant_wallet) = setup();
        test_env.set_caller(admin);
        
        contract.pause_contract();
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            contract.register_merchant(String::from("test"), merchant_wallet);
        }));
        assert!(result.is_err());
        
        contract.unpause_contract();
        contract.register_merchant(String::from("after_unpause"), merchant_wallet);
        assert!(contract.get_merchant(String::from("after_unpause")).is_some());
    }
}
