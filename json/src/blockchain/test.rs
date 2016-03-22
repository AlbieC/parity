// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Blockchain test deserializer.

use std::collections::BTreeMap;
use std::ops::Deref;
use std::io::Read;
use serde_json;
use serde_json::Error;
use blockchain::blockchain::BlockChain;

/// Blockchain test deserializer.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Test(BTreeMap<String, BlockChain>);

impl Deref for Test {
	type Target = BTreeMap<String, BlockChain>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Test {
	/// Loads test from json.
	pub fn load<R>(reader: R) -> Result<Self, Error> where R: Read {
		serde_json::from_reader(reader)
	}
}