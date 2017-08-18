/*
    Generate some strong passwords.
    Copyright (C) 2017  Anthony Defranceschi

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/
//! Here are the password generator features.

extern crate rand;

use self::rand::os;
use std::str::CharIndices;
use std::result::Result;
use std::io;

/// Lower case letters.
const LC: &'static str = "abcdefghijklmnopqrstuvwxyz";
/// Upper case letters.
const UC: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// Special characters.
const SPEC: &'static str = "!?#$_%&*+,./\\:;^~[]";

/// The password generator.
pub struct Curator {
    lc: Option<Vec<char>>,
    uc: Option<Vec<char>>,
    spec: Option<Vec<char>>,
    random: os::OsRng,
}

impl Curator {
    /// Initializes Curator's flags to `None` (by default) and creates an `OsRng`.
    ///
    /// Returns an `Err` if `OsRng::new()` fails.
    pub fn new() -> io::Result<Curator>{
        let randomizer = os::OsRng::new()?;
        Ok(Curator {
            lc: None,
            uc: None,
            spec: None,
            random: randomizer
        })
    }

    /// Enables the lower case letters flag.
    ///
    /// Curator will grab some letters from this string: `abcdefghijklmnopqrstuvwxyz` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_lc(&mut self) -> &Self {
        self.lc = Some(to_char(LC));
        self
    }

    /// Enables the upper case letters flag.
    ///
    /// Curator will grab some letters from this string: `ABCDEFGHIJKLMNOPQRSTUVWXYZ` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_uc(&mut self) -> &Self {
        self.uc = Some(to_char(UC));
        self
    }

    /// Enables the special characters flag.
    ///
    /// Curator will grab some characters from this string: `!?#$_%&*+,./\\:;^~[]` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_spec(&mut self) -> &Self {
        self.spec = Some(to_char(SPEC));
        self
    }
}

/// Transforms a `&str` into `Vec<char>`.
/// ```rust
///     let foo = "hello!";
///     let v = to_char(&foo);
///     assert_eq!(v, vec!['h', 'e', 'l', 'l', 'o', '!']);
/// ```
fn to_char(string: &str) -> Vec<char> {
    let iterator: CharIndices = string.char_indices();
    let mut vector: Vec<char> = Vec::new();
    for letter in iterator {
        vector.push(letter.1);
    }
    vector
}
