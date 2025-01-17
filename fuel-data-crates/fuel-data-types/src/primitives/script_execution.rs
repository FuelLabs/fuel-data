use fuel_core_types::{fuel_asm::RawInstruction, fuel_tx::PanicReason};
use fuel_node::types::*;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PanicInstruction {
    pub reason: PanicReason,
    pub instruction: RawInstruction,
}
impl From<FuelNodePanicInstruction> for PanicInstruction {
    fn from(value: FuelNodePanicInstruction) -> Self {
        Self {
            reason: value.reason().to_owned(),
            instruction: value.instruction().to_owned(),
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
#[repr(u64)]
pub enum ScriptExecutionResult {
    Success,
    Revert,
    Panic,
    // Generic failure case since any u64 is valid here
    GenericFailure(u64),
    #[default]
    Unknown,
}
impl From<FuelNodeScriptExecutionResult> for ScriptExecutionResult {
    fn from(value: FuelNodeScriptExecutionResult) -> Self {
        match value {
            FuelNodeScriptExecutionResult::Success => Self::Success,
            FuelNodeScriptExecutionResult::Revert => Self::Revert,
            FuelNodeScriptExecutionResult::Panic => Self::Panic,
            FuelNodeScriptExecutionResult::GenericFailure(value) => Self::GenericFailure(value),
        }
    }
}
