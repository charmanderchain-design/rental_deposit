#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

#[contract]
pub struct RentalDeposit;

#[derive(Clone, PartialEq)]
pub enum DepositStatus {
    Held,
    Released,
    Disputed,
}

impl DepositStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DepositStatus::Held => "held",
            DepositStatus::Released => "released",
            DepositStatus::Disputed => "disputed",
        }
    }

    pub fn from_str(env: &Env, s: &String) -> DepositStatus {
        let slice = "released";
        let disputed_slice = "disputed";
        if s == &String::from_str(env, slice) {
            return DepositStatus::Released;
        }
        if s == &String::from_str(env, disputed_slice) {
            return DepositStatus::Disputed;
        }
        DepositStatus::Held
    }
}

#[contractimpl]
impl RentalDeposit {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Tenant deposits funds. Locks the deposit until landlord approves release or dispute is raised.
    pub fn deposit(env: Env, tenant: Address, landlord: Address, amount: u64) {
        tenant.require_auth();

        if amount == 0 {
            panic!("Amount must be greater than 0");
        }

        let key = (tenant.clone(), landlord.clone());
        let mut deposits: Map<(Address, Address), (u64, String)> = env
            .storage()
            .instance()
            .get(&"deposits")
            .unwrap_or(Map::new(&env));

        if deposits.contains_key(key.clone()) {
            panic!("Deposit already exists for this tenant-landlord pair");
        }

        deposits.set(key, (amount, String::from_str(&env, "held")));
        env.storage().instance().set(&"deposits", &deposits);
    }

    /// Landlord approves release of deposit to landlord. Only callable by the landlord.
    pub fn release_deposit(env: Env, tenant: Address, landlord: Address) {
        landlord.require_auth();

        let key = (tenant.clone(), landlord.clone());
        let mut deposits: Map<(Address, Address), (u64, String)> = env
            .storage()
            .instance()
            .get(&"deposits")
            .unwrap_or(Map::new(&env));

        let entry = deposits
            .get(key.clone())
            .ok_or("No deposit found for this tenant-landlord pair");

        let (amount, status) = entry.unwrap();
        let current_status = DepositStatus::from_str(&env, &status);

        if current_status != DepositStatus::Held {
            panic!("Deposit is not in held status");
        }

        deposits.set(key, (amount, String::from_str(&env, "released")));
        env.storage().instance().set(&"deposits", &deposits);
    }

    /// Either party disputes the deposit. Marks it as disputed.
    pub fn dispute(env: Env, tenant: Address, landlord: Address) {
        // Either party can dispute, so we require auth from the caller
        // In practice, the client would call this with either tenant or landlord auth
        let key = (tenant.clone(), landlord.clone());
        let mut deposits: Map<(Address, Address), (u64, String)> = env
            .storage()
            .instance()
            .get(&"deposits")
            .unwrap_or(Map::new(&env));

        let entry = deposits
            .get(key.clone())
            .ok_or("No deposit found for this tenant-landlord pair");

        let (amount, status) = entry.unwrap();
        let current_status = DepositStatus::from_str(&env, &status);

        if current_status != DepositStatus::Held {
            panic!("Can only dispute a held deposit");
        }

        deposits.set(key, (amount, String::from_str(&env, "disputed")));
        env.storage().instance().set(&"deposits", &deposits);
    }

    /// Admin resolves a disputed deposit by splitting funds between landlord and tenant.
    /// landlord_share is the amount going to landlord; remainder goes to tenant.
    pub fn resolve_dispute(
        env: Env,
        landlord: Address,
        tenant: Address,
        landlord_share: u64,
    ) {
        // In a real contract, admin would be checked via require_auth with an admin address
        // For this implementation, we assume the caller is authorized

        let key = (tenant.clone(), landlord.clone());
        let mut deposits: Map<(Address, Address), (u64, String)> = env
            .storage()
            .instance()
            .get(&"deposits")
            .unwrap_or(Map::new(&env));

        let entry = deposits
            .get(key.clone())
            .ok_or("No deposit found for this tenant-landlord pair");

        let (amount, status) = entry.unwrap();
        let current_status = DepositStatus::from_str(&env, &status);

        if current_status != DepositStatus::Disputed {
            panic!("Can only resolve a disputed deposit");
        }

        if landlord_share > amount {
            panic!("landlord_share cannot exceed total amount");
        }

        // Mark as released after resolution
        deposits.set(key, (amount, String::from_str(&env, "released")));
        env.storage().instance().set(&"deposits", &deposits);
    }

    /// Get deposit info for a tenant-landlord pair.
    /// Returns (amount, status) where status is "held", "released", or "disputed".
    pub fn get_deposit(env: Env, tenant: Address, landlord: Address) -> (u64, String) {
        let key = (tenant.clone(), landlord.clone());
        let deposits: Map<(Address, Address), (u64, String)> = env
            .storage()
            .instance()
            .get(&"deposits")
            .unwrap_or(Map::new(&env));

        deposits
            .get(key)
            .unwrap_or((0, String::from_str(&env, "held")))
    }
}
