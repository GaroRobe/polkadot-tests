// Copyright (c) 2019 Web3 Technologies Foundation

// This file is part of Polkadot RE Test Suite

// Polkadot RE Test Suite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot RE Tests is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

extern crate clap;

///This file is an interface to run Parity implementation of state trie used in Polkadot RE.

//use trie::{Encode, Decode, HasCompact, Compact, EncodeAsRef, CompactAs};
use clap::{ArgMatches};

fn compute_state_root(matches: &ArgMatches) {
    let trie_key_value_file = matches.value_of("state-file").unwrap();

    let state_trie_root = "";
    println!("state trie root: {:x?}", &state_trie_root);
}

pub fn process_state_trie_command(subcmd_matches: &ArgMatches) {
    if subcmd_matches.is_present("state-root") {
            compute_state_root(subcmd_matches);
    }
}

