
#[cfg(feature = "std")]
use std::fmt;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use std::{
	//collections::HashMap,
	time::SystemTime,
    collections::BTreeMap,
};

use codec::{Codec, Decode, Encode};

use sp_runtime:: {
    traits::{
		self, Block as BlockT, Header as HeaderT, MaybeMallocSizeOf, MaybeSerialize, Member,
		NumberFor, BlakeTwo256,
	},
    Justifications,
};

use sp_core::RuntimeDebug;
use sp_std::prelude::*;

//ToDo: include MaybeSerialize, traits, BlakeTow256.

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct Block<Header, Extrinsic: MaybeSerialize> {
	/// The block header.
	pub header: Header,
	/// The accompanying extrinsics.
	pub extrinsics: Vec<Extrinsic>,
	/// The list of challenges which are solved in this block.
	pub challenges: Vec<u8>,
	/// The list of proposed solution for the chosen challenges.
	pub proposed_solutions: Vec<u8>,//Vec<SolutionsBlock>
}

impl<Header, Extrinsic: MaybeSerialize> traits::Block for Block<Header, Extrinsic>
where
	Header: HeaderT,
	Extrinsic: Member + Codec + traits::Extrinsic + MaybeMallocSizeOf,
{
	type Extrinsic = Extrinsic;
	type Header = Header;
	type Hash = <Self::Header as traits::Header>::Hash;

	fn header(&self) -> &Self::Header {
		&self.header
	}
	fn extrinsics(&self) -> &[Self::Extrinsic] {
		&self.extrinsics[..]
	}
	fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>) {
		print!("Deconstructing block")
		(self.header, self.extrinsics)
	}
	fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>) -> Self {
		print!("Creating new block");
        let challenges = Vec::new();
        let proposed_solutions = Vec::new();
		Block { header, extrinsics, challenges, proposed_solutions }
	}
	fn encode_from(header: &Self::Header, extrinsics: &[Self::Extrinsic]) -> Vec<u8> {
		(header, extrinsics).encode()
	}
}



pub struct SolutionsBlock {
	/// Vector containing the mapping between the ID of the challenge and the propsed solution linked to it.
	pub solution_map: BTreeMap<u32, Solution>,
}

pub struct Solution {
	/// String containing the effective solution, encrypted with its private key and the public key of the retriever.
	pub solution: String,
	/// Hash of the header, transactions, challenge list and solution.
	pub hash: BlakeTwo256,
	/// Timestamp of the generation of the solution.
	pub timestamp: SystemTime,
}

/// Abstraction over a substrate block and justification.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "std", serde(deny_unknown_fields))]
pub struct SignedBlock<Block> {
	/// Full block.
	pub block: Block,
	/// Block justification.
	pub justifications: Option<Justifications>,
}
