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

use aluvm::alu::SiteId;
use aluvm::gfa::FieldInstr;
use aluvm::isa::{CtrlInstr, ReservedInstr};

pub const ISA_ULTRASONIC: &str = "USONIC";

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display, From)]
#[display(inner)]
#[non_exhaustive]
pub enum Instr<Id: SiteId> {
    /// Control flow instructions.
    #[from]
    Ctrl(CtrlInstr<Id>),

    #[from]
    Gfa(FieldInstr),

    #[from]
    Usonic(UsonicInstr),

    /// Reserved instruction for future use in core `ALU` ISAs.
    #[from]
    Reserved(ReservedInstr),
}

impl<Id: SiteId> From<aluvm::gfa::Instr<Id>> for Instr<Id> {
    fn from(instr: aluvm::gfa::Instr<Id>) -> Self {
        match instr {
            aluvm::gfa::Instr::Ctrl(ctrl) => Self::Ctrl(ctrl),
            aluvm::gfa::Instr::Gfa(gfa) => Self::Gfa(gfa),
            aluvm::gfa::Instr::Reserved(resrv) => Self::Reserved(resrv),
            _ => unreachable!(),
        }
    }
}

/// The instruction set uses iterator semantics and not random access semantic to correspond to the
/// RISC type of the machine and not to add assumptions about abilities to access the operation
/// state in a random way. Operation state is always iterated, such that not a single state
/// element can be missed (as long as iterator runs to the end).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(inner)]
#[non_exhaustive]
pub enum UsonicInstr {
    /// Checks whether there is a next destructible memory cell in the contract state listed in the
    /// operation input and sets `CO` register accordingly.
    #[display("cknxi   :destructible")]
    CkNxIRo,

    /// Checks whether there is a next immutable memory cell in the contract state listed in the
    /// operation input and sets `CO` register accordingly.
    #[display("cknxi   :immutable")]
    CkNxIAo,

    /// Checks whether there is a next destructible memory cell defined by the operation and sets
    /// `CO` register accordingly.
    #[display("cknxo   :destructible")]
    CkNxORo,

    /// Checks whether there is a next immutable memory cell defined by the operation and sets `CO`
    /// register accordingly.
    #[display("cknxo   :immutable")]
    CkNxOAo,

    /// Load next [`StateValue`] from the current input destructible memory cell to `EA`-`ED`
    /// registers.
    ///
    /// If the next state value is absent, sets `CO` to a failed state. Otherwise, resets `CO`.
    #[display("ldi     :destructible")]
    LdIRo,

    /// Load next [`StateValue`] from the current input immutable memory cell to `EA`-`ED`
    /// registers.
    ///
    /// If the next state value is absent, sets `CO` to a failed state. Otherwise, resets `CO`.
    #[display("ldi     :immutable")]
    LdIAo,

    /// Load next [`StateValue`] from the current output destructible memory cell to `EA`-`ED`
    /// registers.
    ///
    /// If the next state value is absent, sets `CO` to a failed state. Otherwise, resets `CO`.
    #[display("ldo     :destructible")]
    LdORo,

    /// Load next [`StateValue`] from the current output immutable memory cell to `EA`-`ED`
    /// registers.
    ///
    /// If the next state value is absent, sets `CO` to a failed state. Otherwise, resets `CO`.
    #[display("ldo     :immutable")]
    LdOAo,

    /// Resets iterator over input destructible memory cells by setting corresponding `UI` value to
    /// zero.
    ///
    /// Does not affect the value of `CO` or `CK` registers.
    #[display("rsti    :destructible")]
    RstIRo,

    /// Resets iterator over input immutable memory cells by setting corresponding `UI` value to
    /// zero.
    ///
    /// Does not affect the value of `CO` or `CK` registers.
    #[display("rsti    :immutable")]
    RstIAo,

    /// Resets iterator over output destructible memory cells by setting corresponding `UI` value
    /// to zero.
    ///
    /// Does not affect the value of `CO` or `CK` registers.
    #[display("rsto    :destructible")]
    RstORo,

    /// Resets iterator over output immutable memory cells by setting corresponding `UI` value to
    /// zero.
    ///
    /// Does not affect the value of `CO` or `CK` registers.
    #[display("rsto    :immutable")]
    RstOAo,
}
