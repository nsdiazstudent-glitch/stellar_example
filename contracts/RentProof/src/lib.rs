
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env,
};

#[contracttype]
#[derive(Clone)]
pub struct RentalEscrow {
    pub tenant: Address,
    pub landlord: Address,
    pub deposit_amount: i128,
    pub released: bool,
}

#[contracttype]
pub enum StorageKey {
    Escrow(Address),
}

#[contract]
pub struct BarangayRentSafeContract;

#[contractimpl]
impl BarangayRentSafeContract {

    // Tenant creates escrow deposit
    pub fn create_escrow(
        env: Env,
        tenant: Address,
        landlord: Address,
        amount: i128,
    ) {

        tenant.require_auth();

        let escrow = RentalEscrow {
            tenant: tenant.clone(),
            landlord,
            deposit_amount: amount,
            released: false,
        };

        env.storage().instance().set(
            &StorageKey::Escrow(tenant),
            &escrow,
        );
    }

    // Release escrow after move-out
    pub fn release_escrow(
        env: Env,
        landlord: Address,
        tenant: Address,
    ) {

        landlord.require_auth();

        let key =
            StorageKey::Escrow(tenant.clone());

        let mut escrow: RentalEscrow =
            env.storage()
                .instance()
                .get(&key)
                .unwrap();

        if escrow.released {
            panic!("escrow already released");
        }

        if escrow.landlord != landlord {
            panic!("unauthorized landlord");
        }

        escrow.released = true;

        env.storage().instance().set(
            &key,
            &escrow,
        );
    }

    // View escrow details
    pub fn get_escrow(
        env: Env,
        tenant: Address,
    ) -> RentalEscrow {

        env.storage()
            .instance()
            .get(&StorageKey::Escrow(tenant))
            .unwrap()
    }
}