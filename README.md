# Substrate Cumulus Parachain for BlockScore®

1. What is the BlockScore® Protocol?

This is "Data Scoring Protocol - DSP", the standardized metric of the user’s creditworthiness. It is an unbiased, transparent, and decentralized scoring model similar to VantageScore or FICO while allowing the implementation of alternative data and economic models to more effectively complete the financial identity. This is governed by the non-profit organization, the BlockScore® Network.

The BlockScore® Network is a transparent, decentralized blockchain supporting the BlockScore® Protocol is a dynamic indicator of an individual’s creditworthiness, adapting to the maturity of the user’s credit history whether they are new to credit or have been building for years. The credit algorithms can interact with the users’ centralized financial institution, alternative data providers via the side-chain operators, and on-chain assets and historical behavior to assess the risk of default before generating a credit score. 

The BlockScore® Network is an inclusive credit scoring system for verified financial identity. The protocol allows users to establish a global, federated identity with third parties and whitelisted partners who “vouch” for their identity information, legal status, and creditworthiness. The BlockID® dNFT is the data management protocol for establishing a reliable identity and forms the BlockScore® Network of creditworthy users. 


2. How can we use the BlockScore® Protocol?

This is the parachain for BlockScore®, which is to calculate and to store the credit score of each user who owns a NFT after registering the credit information.

Here are 3 structures for operate the protocol;

    - CreditData
        The following five factors are vital to achieving and maintaining a good BlockScore®, no matter which score is being used. The percentages listed refer to how heavily these factors weigh into a consumer’s BlockScore®.

        pub struct CreditData {
            pub payment_history: i32,           //35%
            pub credit_utilization: i32,        //30%
            pub credit_age: i32,                //15%
            pub number_of_inquires: i32,        //10%
            pub types_of_credit: i32,           //10%
        }

    - ScoreFactor
        The following three factors are used when we calculate the credit score of the users in various situations.

        pub struct ScoreFactor {
            pub financial_score: i32,           //40 - 45%
            pub reliability_score: i32,         //45 - 55%
            pub social_score: i32,              //15%
        }

    - ScoreRange
        There are various credit scoring formulas currently in use, but most operate on a scale of 300–850. Each credit score has its own scoring model on the same basic concept. The FICO score is the most commonly used scoring method maintained by the Fair Isaac Corporation. With BlockScore®, the range will be 0-100. This is a globally understood range leading to faster adoption. Each level will include a cap based on data provided to prove an individual’s financial identity.

        pub struct ScoreRange {
            pub BAD: string,                    //0 - 46
            pub POOR: string,                   //47 - 64
            pub FAIR: string,                   //65 - 73
            pub GOOD: string,                   //74 - 89
            pub EXCELLENT: string,              //90 - 100
        }

The functions the users can call via Polkadot.js are kinds of "Set/Get" named as below;
    Set : "set_score", "set_credit_data", "set_score_factor", "set_score_range"
    Get : "get_score", "get_credit_data", "get_score_factor", "get_score_range"