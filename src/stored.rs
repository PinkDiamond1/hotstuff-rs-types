/*
 Copyright 2022 ParallelChain Lab

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 */

use std::collections::{hash_set, HashSet, HashMap, hash_map};
use borsh::{BorshSerialize, BorshDeserialize};
use crate::messages::BlockHash;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct ChildrenList(HashSet<BlockHash>);

impl ChildrenList {
    pub fn new() -> ChildrenList {
        ChildrenList(HashSet::new())
    }

    pub fn insert(&mut self, child_hash: BlockHash) {
        self.0.insert(child_hash);
    }

    pub fn iter(&self) -> hash_set::Iter<BlockHash> {
        self.0.iter()
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct StorageMutations {
    inserts: HashMap<Key, Value>,
    deletes: HashSet<Key>, 
}
pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

impl StorageMutations {
    pub fn new() -> StorageMutations {
        StorageMutations {
            inserts: HashMap::new(),
            deletes: HashSet::new(),
        }
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        self.deletes.remove(&key);
        self.inserts.insert(key, value);
    }

    pub fn delete(&mut self, key: Key) {
        self.inserts.remove(&key);
        self.deletes.insert(key);
    }

    pub fn get_insert(&self, key: &Key) -> Option<&Value> {
        self.inserts.get(key)
    } 

    pub fn contains_delete(&self, key: &Key) -> bool {
        self.deletes.contains(key)
    }

    /// Get an iterator over all of the key, value pairs in this WriteSet.
    pub fn inserts(&self) -> hash_map::Iter<Key, Value> {
        self.inserts.iter()
    } 

    /// Get an iterator over all of the keys that are deleted by this WriteSet.
    pub fn deletes(&self) -> hash_set::Iter<Key> {
        self.deletes.iter()
    } 
}

pub type DataLen = u64;
