#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short, log, Map};

// Data structure for storing staking information
#[contracttype]
#[derive(Clone)]
pub struct StakeInfo {
    pub staked_at: u64,
    pub nft_id: u64,
    pub is_active: bool,
}

// Symbol for mapping
const STAKEBOOK: Symbol = symbol_short!("STAKEBOOK");

#[contract]
pub struct NFTStaking;

#[contractimpl]
impl NFTStaking {
    // Stake NFT
    pub fn stake(env: Env, user: Address, nft_id: u64) {
        let timestamp = env.ledger().timestamp();
        let mut stakes: Map<u64, StakeInfo> = env
            .storage()
            .instance()
            .get(&STAKEBOOK)
            .unwrap_or(Map::new(&env));

        // Check if NFT is already staked
        if stakes.contains_key(&nft_id) {
            panic!("NFT is already staked");
        }

        let info = StakeInfo {
            staked_at: timestamp,
            nft_id,
            is_active: true,
        };

        stakes.set(nft_id, info);
        env.storage().instance().set(&STAKEBOOK, &stakes);
        log!(&env, "NFT {} staked by {}", nft_id, user);
    }

    // Unstake NFT
    pub fn unstake(env: Env, nft_id: u64) {
        let mut stakes: Map<u64, StakeInfo> = env
            .storage()
            .instance()
            .get(&STAKEBOOK)
            .unwrap_or(Map::new(&env));

        if let Some(mut stake) = stakes.get(nft_id) {
            if stake.is_active {
                stake.is_active = false;
                stakes.set(nft_id, stake);
                env.storage().instance().set(&STAKEBOOK, &stakes);
                log!(&env, "NFT {} unstaked", nft_id);
            } else {
                panic!("NFT already unstaked");
            }
        } else {
            panic!("NFT not found");
        }
    }

    // View staking info for NFT
    pub fn view_stake(env: Env, nft_id: u64) -> StakeInfo {
        let stakes: Map<u64, StakeInfo> = env
            .storage()
            .instance()
            .get(&STAKEBOOK)
            .unwrap_or(Map::new(&env));

        stakes.get(nft_id).unwrap_or(StakeInfo {
            staked_at: 0,
            nft_id,
            is_active: false,
        })
    }
}
