#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};

use crate::{
    BarangayRentSafeContract,
    BarangayRentSafeContractClient,
};

mod tests {

    use super::*;

    #[test]
    fn test_happy_path() {

        let env = Env::default();

        let contract_id =
            env.register_contract(
                None,
                BarangayRentSafeContract,
            );

        let client =
            BarangayRentSafeContractClient::new(
                &env,
                &contract_id,
            );

        let tenant =
            Address::generate(&env);

        let landlord =
            Address::generate(&env);

        client.create_escrow(
            &tenant,
            &landlord,
            &5000,
        );

        client.release_escrow(
            &landlord,
            &tenant,
        );

        let escrow =
            client.get_escrow(&tenant);

        assert_eq!(escrow.released, true);
    }

    #[test]
    #[should_panic(expected = "unauthorized landlord")]
    fn test_edge_case_wrong_landlord() {

        let env = Env::default();

        let contract_id =
            env.register_contract(
                None,
                BarangayRentSafeContract,
            );

        let client =
            BarangayRentSafeContractClient::new(
                &env,
                &contract_id,
            );

        let tenant =
            Address::generate(&env);

        let landlord =
            Address::generate(&env);

        let fake_landlord =
            Address::generate(&env);

        client.create_escrow(
            &tenant,
            &landlord,
            &5000,
        );

        client.release_escrow(
            &fake_landlord,
            &tenant,
        );
    }

    #[test]
    fn test_state_verification() {

        let env = Env::default();

        let contract_id =
            env.register_contract(
                None,
                BarangayRentSafeContract,
            );

        let client =
            BarangayRentSafeContractClient::new(
                &env,
                &contract_id,
            );

        let tenant =
            Address::generate(&env);

        let landlord =
            Address::generate(&env);

        client.create_escrow(
            &tenant,
            &landlord,
            &7000,
        );

        let escrow =
            client.get_escrow(&tenant);

        assert_eq!(
            escrow.deposit_amount,
            7000,
        );

        assert_eq!(
            escrow.released,
            false,
        );
    }

    #[test]
    #[should_panic(expected = "escrow already released")]
    fn test_duplicate_release() {

        let env = Env::default();

        let contract_id =
            env.register_contract(
                None,
                BarangayRentSafeContract,
            );

        let client =
            BarangayRentSafeContractClient::new(
                &env,
                &contract_id,
            );

        let tenant =
            Address::generate(&env);

        let landlord =
            Address::generate(&env);

        client.create_escrow(
            &tenant,
            &landlord,
            &4000,
        );

        client.release_escrow(
            &landlord,
            &tenant,
        );

        client.release_escrow(
            &landlord,
            &tenant,
        );
    }

    #[test]
    fn test_multiple_escrows() {

        let env = Env::default();

        let contract_id =
            env.register_contract(
                None,
                BarangayRentSafeContract,
            );

        let client =
            BarangayRentSafeContractClient::new(
                &env,
                &contract_id,
            );

        let tenant1 =
            Address::generate(&env);

        let tenant2 =
            Address::generate(&env);

        let landlord1 =
            Address::generate(&env);

        let landlord2 =
            Address::generate(&env);

        client.create_escrow(
            &tenant1,
            &landlord1,
            &5000,
        );

        client.create_escrow(
            &tenant2,
            &landlord2,
            &8000,
        );

        let escrow1 =
            client.get_escrow(&tenant1);

        let escrow2 =
            client.get_escrow(&tenant2);

        assert_eq!(
            escrow1.deposit_amount,
            5000,
        );

        assert_eq!(
            escrow2.deposit_amount,
            8000,
        );
    }
}