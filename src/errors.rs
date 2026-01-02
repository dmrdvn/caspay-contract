use odra::prelude::*;

/// Custom error types for CasPay contract operations
#[odra::odra_error]
pub enum CasPayError {
    /// Unauthorized access attempt - caller is not authorized
    Unauthorized = 1000,
    
    /// Invalid merchant - merchant does not exist or is inactive
    InvalidMerchant = 1001,
    
    /// Merchant already registered with this ID
    MerchantAlreadyExists = 1002,
    
    /// Invalid product - product does not exist or is inactive
    InvalidProduct = 1003,
    
    /// Product already exists with this ID
    ProductAlreadyExists = 1004,
    
    /// Invalid subscription plan
    InvalidSubscriptionPlan = 1005,
    
    /// Subscription plan already exists
    SubscriptionPlanAlreadyExists = 1006,
    
    /// Invalid payment - amount, token or merchant mismatch
    InvalidPayment = 1007,
    
    /// Payment already recorded with this transaction hash
    PaymentAlreadyRecorded = 1008,
    
    /// Invalid subscription - subscription does not exist or expired
    InvalidSubscription = 1009,
    
    /// Subscription already active for this user and plan
    SubscriptionAlreadyActive = 1010,
    
    /// Insufficient balance for withdrawal
    InsufficientBalance = 1011,
    
    /// Invalid token address
    InvalidToken = 1012,
    
    /// Contract is paused - operations disabled
    ContractPaused = 1013,
    
    /// Invalid timestamp - must be in the future
    InvalidTimestamp = 1014,
    
    /// Invalid amount - must be greater than zero
    InvalidAmount = 1015,
    
    /// Invalid merchant status
    InvalidMerchantStatus = 1018,
    
    /// Cannot delete - entity has dependencies
    HasDependencies = 1019,
    
    /// Storage overflow - too many entities
    StorageOverflow = 1020,
    
    /// Invalid signature
    InvalidSignature = 1021,
    
    /// Operation expired
    OperationExpired = 1022,
}
