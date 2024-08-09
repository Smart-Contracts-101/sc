#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod staking_contract {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct StakingContract {
        stakes: StorageHashMap<AccountId, Balance>,
        total_staked: Balance,
    }

    impl StakingContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                stakes: StorageHashMap::new(),
                total_staked: 0,
            }
        }

        #[ink(message)]
        pub fn stake(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let current_stake = self.stakes.get(&caller).copied().unwrap_or(0);
            self.stakes.insert(caller, current_stake + amount);
            self.total_staked += amount;
        }

        #[ink(message)]
        pub fn unstake(&mut self, amount: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let current_stake = self.stakes.get(&caller).copied().unwrap_or(0);
            if current_stake < amount {
                return Err("Not enough staked");
            }
            self.stakes.insert(caller, current_stake - amount);
            self.total_staked -= amount;
            Ok(())
        }

        #[ink(message)]
        pub fn get_stake(&self, account: AccountId) -> Balance {
            self.stakes.get(&account).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_staked(&self) -> Balance {
            self.total_staked
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn staking_works() {
            let mut contract = StakingContract::new();
            contract.stake(100);
            assert_eq!(contract.get_stake(contract.env().caller()), 100);
            assert_eq!(contract.get_total_staked(), 100);
        }

        #[ink::test]
        fn unstaking_works() {
            let mut contract = StakingContract::new();
            contract.stake(100);
            assert_eq!(contract.unstake(50), Ok(()));
            assert_eq!(contract.get_stake(contract.env().caller()), 50);
            assert_eq!(contract.get_total_staked(), 50);
        }

        #[ink::test]
        fn unstaking_fails_when_not_enough_staked() {
            let mut contract = StakingContract::new();
            contract.stake(100);
            assert_eq!(contract.unstake(150), Err("Not enough staked"));
            assert_eq!(contract.get_stake(contract.env().caller()), 100);
            assert_eq!(contract.get_total_staked(), 100);
        }
    }
}
