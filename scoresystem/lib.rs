#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
// use parity_scale_codec::alloc::string::ToString;

#[ink::contract]
mod scoresystem {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Scoresystem {
        score: i32,
        credit_data: i32,
        score_factor: i32;
        score_range: i32,
    }

    impl Scoresystem {
        #[ink(constructor)]
        pub fn new(init_score: i32, init_weight: i32, init_factor: i32, init_result: i32) -> Self {
            Self {
                score: init_score,
                credit_data: init_weight,
                score_factor: init_factor,
                score_range: init_result,
            }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                score: Default::default(),
                credit_data: Default::default(),
                score_factor: Default::default(),
                score_range: Default::default(),
            }
        }
        
        #[ink(message)]
        pub fn set_credit_data(&mut self, credit_data_tmp: i32) {
            self.credit_data = credit_data_tmp;
        }
        
        #[ink(message)]
        pub fn get_credit_data(&self) -> i32 {
            self.credit_data
        }

        #[ink(message)]
        pub fn set_score_factor(&mut self, score_factor_tmp: i32) {
            self.score_factor = score_factor_tmp;
        }

        #[ink(message)]
        pub fn get_score_factor(&self) -> i32 {
            self.score_factor
        }

        #[ink(message)]
        pub fn set_score_range(&mut self, score_range_tmp: i32) {
            self.score_range = score_range_tmp;
        }
        
        #[ink(message)]
        pub fn get_score_range(&self) -> i32 {
            self.score_range
        }

        #[ink(message)]
        pub fn set_score(&mut self, score_tmp: i32) {
            self.score = score_tmp;
        }

        #[ink(message)]
        pub fn get_score(&self) -> i32 {
            self.score
        }
    }

    // #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
    // pub struct CreditData {
    //     pub payment_history: i32,
    //     pub credit_utilization: i32,
    //     pub credit_age: i32,
    //     pub number_of_inquires: i32,
    //     pub types_of_credit: i32,
    // }

    // #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
    // pub struct ScoreFactor {
    //     pub financial_score: i32,
    //     pub reliability_score: i32,
    //     pub social_score: i32,
    // }

    // #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
    // pub struct ScoreRange {
    //     pub BAD: string,
    //     pub POOR: string,
    //     pub FAIR: string,
    //     pub GOOD: string,
    //     pub EXCELLENT: string,
    // }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Scoresystem::default();
            assert_eq!(contract.get_score(), 0);
            assert_eq!(contract.get_credit_data(), 0);
            assert_eq!(contract.get_score_factor(), 0);
            assert_eq!(contract.get_score_range(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Scoresystem::new(42, 10, 0);
            assert_eq!(contract.get_score(), 42);
            contract.set_score(5);
            assert_eq!(contract.get_credit_data(), 10);
            contract.set_credit_data(-50);
            assert_eq!(contract.get_score_factor(), 15);
            contract.set_score_factor(20);
            assert_eq!(contract.get_score_range(), 0);
            contract.set_score_range(2);
        }
    }
}
