// This file is part of CCcc, A simple x86-64 compiler for a tiny subset of C.
// Copyright (C) 2026 CordlessCoder
//
// CCcc is free software: you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// CCcc is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with CCcc.
// If not, see <https://www.gnu.org/licenses/>.

use std::borrow::Cow;

use utils::VarInt;


fn remove_underscores(s: &str) -> Cow<'_, str> {
    s.split('_')
        .map(Cow::from)
        .reduce(|a, b| (a.into_owned() + &b).into())
        .unwrap_or_default()
}

pub fn parse_int(base: u32, text: &str) -> Option<VarInt> {
    let text = remove_underscores(text);
    u64::from_str_radix(&text, base)
        .map(VarInt::Pos)
        .ok()
        .or_else(|| i64::from_str_radix(&text, base).map(VarInt::Neg).ok())
}
