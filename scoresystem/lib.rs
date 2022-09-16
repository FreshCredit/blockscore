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
        score_weight: i32,
        score_result: i32,
    }

    impl Scoresystem {
        #[ink(constructor)]
        pub fn new(init_score: i32, init_weight: i32, init_result: i32) -> Self {
            Self {
                score: init_score,
                score_weight: init_weight,
                score_result: init_result,
            }
        }

        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                score: Default::default(),
                score_weight: Default::default(),
                score_result: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get_score(&self) -> i32 {
            self.score
        }

        #[ink(message)]
        pub fn set_score(&mut self, score_tmp: i32) {
            self.score = score_tmp;
        }

        #[ink(message)]
        pub fn get_score_weight(&self) -> i32 {
            self.score_weight
        }

        #[ink(message)]
        pub fn set_score_weight(&mut self, score_weight_tmp: i32) {
            self.score_weight = score_weight_tmp;
        }

        #[ink(message)]
        pub fn get_score_result(&self) -> i32 {
            self.score_result
        }

        #[ink(message)]
        pub fn set_score_result(&mut self, score_result_tmp: i32) {
            self.score_result = score_result_tmp;
        }
    }

    // #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
    // pub struct ScoreWeight {
    //     pub PaymentHistory: i32,
    //     pub CreditUtilization: i32,
    //     pub CreditAge: i32,
    //     pub NumberOfInquires: i32,
    //     pub TypesOfCredit: i32,
    // }

    // #[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
    // pub struct ScoreResult {
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
            assert_eq!(contract.get_score_weight(), 0);
            assert_eq!(contract.get_score_result(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Scoresystem::new(42, 10, 0);
            assert_eq!(contract.get_score(), 42);
            contract.set_score(5);
            assert_eq!(contract.get_score_weight(), 10);
            contract.set_score_weight(-50);
            assert_eq!(contract.get_score_result(), 0);
            contract.set_score_result(2);
        }
    }
}
