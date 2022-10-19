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

/// Defines types and encodings that are used in messages exchanged between Participants. The wire-encodings of these types are an
/// integral part of the protocol, and all Participant implementations that wish to communicate with the implementation in this crate
/// needs to encode these types in the exact same way.
pub mod messages;

/// Defines *additional* types that the Block Tree's storage implementation needs to store to support its functionality, but which are
/// not sent over the wire and therefore are not part of the protocol. An example functionality of the Block Tree that requires these
/// additional types is getting the child of a Block, since a msg_types::Block only stores a pointer to its parent in the form of 
/// `justify.block_hash`.
pub mod stored;

/// Defines types that give a cryptographic identity to every Participant. This chiefly includes secret keys, public keys, and signatures.
pub mod identity;
