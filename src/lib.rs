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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// TODO: Activate no_std once StrictEncoding will support it
// #![no_std]

extern crate alloc;

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[macro_use]
extern crate commit_verify;
extern crate zkaluvm as aluvm;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod codex;
mod state;
mod operation;
mod isa;
mod contract;
#[cfg(feature = "stl")]
pub mod stl;
mod util;

use amplify::num::u256;
pub use codex::{AccessId, CallError, CallId, Codex, CodexId, LibRepo, Memory, VmContext};
pub use contract::{Consensus, ConstU32, Contract, ContractId, ContractMeta, ContractName};
pub use isa::{Instr, UsonicCore, UsonicInstr, ISA_ULTRASONIC};
#[cfg(feature = "baid64")]
pub use operation::ParseAddrError;
pub use operation::{CellAddr, Genesis, GenesisId, Input, Operation, Opid};
pub use state::{AuthToken, RawData, StateCell, StateData, StateValue};
pub use util::Identity;
pub use zkaluvm::fe256;

pub const LIB_NAME_ULTRASONIC: &str = "UltraSONIC";

pub const FIELD_ORDER_25519: u256 = u256::from_inner([
    0xFFFF_FFFF_FFFF_FFEC,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x8FFF_FFFF_FFFF_FFFF,
]);
pub const FIELD_ORDER_STARK: u256 = u256::from_inner([1, 0, 17, 0x0800_0000_0000_0000]);
pub const FIELD_ORDER_SECP: u256 = u256::from_inner([
    0xFFFF_FFFE_FFFF_FC2E,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
]);
