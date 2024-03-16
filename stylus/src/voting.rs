use stylus_sdk::{msg, prelude::*};
use stylus_sdk::storage::{StorageAddress, StorageMap, StorageString};

#[solidity_storage]
#[entrypoint]
pub struct Voting {
    votes: StorageMap<StorageAddress, StorageString>,
}

#[external]
impl Voting {
    pub fn vote(&mut self, vote: StorageString) {
        self.votes.insert(msg::sender(), vote);
    }

    pub fn get_vote(&self) -> StorageString {
        self.votes.get(msg::sender()).unwrap_or_default()
    }
}
