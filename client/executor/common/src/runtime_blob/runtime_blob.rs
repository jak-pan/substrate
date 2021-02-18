// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use parity_wasm::elements::{deserialize_buffer, DataSegment, Module as RawModule};

/// A bunch of information collected from a WebAssembly module.
pub struct RuntimeBlob {
	raw_module: RawModule,
}

impl RuntimeBlob {
	/// Create `RuntimeBlob` from the given wasm code.
	///
	/// Returns `None` if the wasm code cannot be deserialized.
	pub fn new(wasm_code: &[u8]) -> Option<Self> {
		let raw_module: RawModule = deserialize_buffer(wasm_code).ok()?;
		Some(Self { raw_module })
	}

	/// Extract the data segments from the given wasm code.
	///
	/// Returns `Err` if the given wasm code cannot be deserialized.
	pub(super) fn data_segments(&self) -> Vec<DataSegment> {
		self.raw_module
			.data_section()
			.map(|ds| ds.entries())
			.unwrap_or(&[])
			.to_vec()
	}

	/// The number of globals defined in locally in this module.
	pub fn declared_globals_count(&self) -> u32 {
		self.raw_module
			.global_section()
			.map(|gs| gs.entries().len() as u32)
			.unwrap_or(0)
	}

	/// The number of imports of globals.
	pub fn imported_globals_count(&self) -> u32 {
		self.raw_module
			.import_section()
			.map(|is| is.globals() as u32)
			.unwrap_or(0)
	}

	/// Make sure that the mutable globals are exported
	pub fn expose_mutable_globals(&mut self) {
		pwasm_utils::export_mutable_globals(&mut self.raw_module, "exported_internal_global");
	}
}