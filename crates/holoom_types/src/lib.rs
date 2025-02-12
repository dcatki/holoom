use hdi::prelude::*;
use serde::{Deserialize, Serialize};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct UsernameAttestation {
    pub agent: AgentPubKey,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedUsername {
    pub username: String,
    pub signature: Signature,
    pub signer: AgentPubKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataItem {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMetadataItemPayload {
    pub agent_pubkey: AgentPubKey,
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMetadataItemValuePayload {
    pub agent_pubkey: AgentPubKey,
    pub name: String,
}

pub type EvmAddress = alloy_primitives::Address;
pub type EvmSignature = alloy_primitives::Signature;
pub type SolanaAddress = ed25519_dalek::VerifyingKey;
pub type SolanaSignature = ed25519_dalek::Signature;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChainWalletSignature {
    Evm {
        evm_address: EvmAddress,
        evm_signature: EvmSignature,
    },
    Solana {
        solana_address: SolanaAddress,
        solana_signature: SolanaSignature,
    },
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct WalletAttestation {
    pub agent: AgentPubKey,
    pub chain_wallet_signature: ChainWalletSignature,
    pub prev_action: ActionHash,
}

#[derive(Serialize, Deserialize, Debug, Clone, SerializedBytes)]
pub struct SignableBytes(pub Vec<u8>);

#[dna_properties]
#[derive(Clone)]
pub struct HoloomDnaProperties {
    pub authority_agent: String,
}

pub fn get_authority_agent() -> ExternResult<AgentPubKey> {
    let dna_props = HoloomDnaProperties::try_from_dna_properties()?;
    AgentPubKey::try_from(dna_props.authority_agent).map_err(|_| {
        wasm_error!(WasmErrorInner::Guest(
            "Failed to deserialize AgentPubKey from dna properties".into()
        ))
    })
}
