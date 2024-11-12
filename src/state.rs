// UltraSONIC: transactional execution layer with capability-based memory access for zk-AluVM
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2019-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use aluvm::{fe128, LibSite};
use commit_verify::MerkleHash;

use crate::LIB_NAME_ULTRASONIC;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
#[derive(CommitEncode)]
#[commit_encode(strategy = strict, id = MerkleHash)]
#[derive(StrictType, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_ULTRASONIC, tags = custom)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(untagged, rename_all = "camelCase")
)]
pub enum StateData {
    #[default]
    #[strict_type(tag = 0x00)]
    None,
    #[strict_type(tag = 0x01)]
    Single(fe128),
    #[strict_type(tag = 0x02)]
    Double(fe128, fe128),
    #[strict_type(tag = 0x03)]
    Three(fe128, fe128, fe128),
    #[strict_type(tag = 0x04)]
    Four(fe128, fe128, fe128, fe128),
}

impl StateData {
    pub fn get(&self, pos: u8) -> Option<fe128> {
        match (*self, pos) {
            (Self::Single(el), 0)
            | (Self::Double(el, _), 0)
            | (Self::Three(el, _, _), 0)
            | (Self::Four(el, _, _, _), 0) => Some(el),

            (Self::Double(_, el), 1)
            | (Self::Three(_, el, _), 1)
            | (Self::Four(_, el, _, _), 1) => Some(el),

            (Self::Three(_, _, el), 2) | (Self::Four(_, _, el, _), 2) => Some(el),

            (Self::Four(_, _, _, el), 3) => Some(el),

            _ => None,
        }
    }
}

/// Read-once access-controlled memory cell.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[derive(CommitEncode)]
#[commit_encode(strategy = strict, id = MerkleHash)]
#[derive(StrictType, StrictDumb, StrictEncode, StrictDecode)]
#[strict_type(lib = LIB_NAME_ULTRASONIC)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub struct StateCell {
    pub data: StateData,
    pub seal: fe128,
    pub lock: Option<LibSite>,
}
