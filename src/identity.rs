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

use std::collections::BTreeMap;
use std::net::IpAddr;
use ed25519_dalek::{Keypair as DalekKeypair, PublicKey as DalekPublicKey};

/// A set of (PublicAddr, IpAddr) pairs, indexable by PublicAddr. It is a BTreeMap instead of, say, a HashMap, so that we can
/// we can iterate through it in ascending order of PublicAddr. This implifies SignatureSet verification.
pub type ParticipantSet = BTreeMap<PublicKeyBytes, IpAddr>;

/// An Ed25519 Keypair. This is used by the Participant to sign Vote messages.
pub type KeyPair = DalekKeypair;

/// An Ed25519 Public Key. This uniquely identifies Participants.
pub type PublicKey = DalekPublicKey;

/// The backing byte-sequence behind PublicKey. This is defined separately because PublicKey does not implement Hash, which we
/// need to use it as a key in the [IndexMaps](indexmap::IndexMap) used in IPC.
pub type PublicKeyBytes = [u8; 32];

