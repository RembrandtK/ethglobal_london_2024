use stylus_sdk::{msg, prelude::*};
use stylus_sdk::prelude::sol_storage;

sol_storage! {
    pub struct Voting {
        mapping(address => string) votes;
    }
}

// #[solidity_storage]
// #[entrypoint]
// pub struct Voting {
//     votes: StorageMap<StorageKey, StorageString>,
// }

#[external]
impl Voting {
    pub fn vote(&mut self, vote: String) {
        self.votes.insert(msg::sender(), vote.into());
    }

    pub fn get_vote(&self) -> String {
        self.votes.get(msg::sender())
    }
}
