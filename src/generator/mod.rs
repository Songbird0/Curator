/*
    Generates some strong passwords.
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
use self::rand::Rng;
use std::str::CharIndices;
use std::io;

/// Integers.
const INT: &'static str = "0123456789";
/// Lower case letters.
const LC: &'static str = "abcdefghijklmnopqrstuvwxyz";
/// Upper case letters.
const UC: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
/// Special characters.
const SPEC: &'static str = "!?#$_%&*+,./\\:;^~[]";

/// The password generator.
pub struct Curator {
    integer: Option<Vec<char>>,
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
            integer: None,
            lc: None,
            uc: None,
            spec: None,
            random: randomizer
        })
    }

    /// Enables the integers flag.
    ///
    /// Curator will grab some integers from this string: `0123456789` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_integer(&mut self) -> &mut Self {
        self.integer = Some(to_char(INT));
        self
    }

    /// Enables the lower case letters flag.
    ///
    /// Curator will grab some letters from this string: `abcdefghijklmnopqrstuvwxyz` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_lc(&mut self) -> &mut Self {
        self.lc = Some(to_char(LC));
        self
    }

    /// Enables the upper case letters flag.
    ///
    /// Curator will grab some letters from this string: `ABCDEFGHIJKLMNOPQRSTUVWXYZ` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_uc(&mut self) -> &mut Self {
        self.uc = Some(to_char(UC));
        self
    }

    /// Enables the special characters flag.
    ///
    /// Curator will grab some characters from this string: `!?#$_%&*+,./\\:;^~[]` to generate a password.
    /// > **Note**: Curator's flags may be combined together.
    pub fn enable_spec(&mut self) -> &mut Self {
        self.spec = Some(to_char(SPEC));
        self
    }

    /// Generates a single password.
    pub fn gen_pwd(&mut self, occcurrences: usize) -> String {
        let mut enabled_flags: Vec<&Vec<char>> = Vec::with_capacity(3); // max 3
        let mut one_flag_at_least: bool = false;
        // The following characters will be randomly picked at generation-time.
        if let Some(ref int_flag) = self.integer {
            enabled_flags.push(int_flag);
            one_flag_at_least |= true;
        }
        if let Some(ref lc_flag) = self.lc {
            enabled_flags.push(lc_flag);
            one_flag_at_least |= true;
        }
        if let Some(ref uc_flag) = self.uc {
            enabled_flags.push(uc_flag);
            one_flag_at_least |= true;
        }
        if let Some(ref spec_flag) = self.spec {
            enabled_flags.push(spec_flag);
            one_flag_at_least |= true;
        }
        if !one_flag_at_least {
            panic!("`gen_pwd` cannot be used if no flags were activated.");
        }
        let ef_length: usize = enabled_flags.len();
        let mut str_pwd = String::with_capacity(occcurrences);
        for _ in 0..occcurrences {
            let picked_flag_id: u32 = self.random.gen_range(0, ef_length as u32);
            let picked_vec: &Vec<char> = enabled_flags[picked_flag_id as usize];
            let vec_length: usize = picked_vec.len();
            let char_id: u32 = self.random.gen_range(0, vec_length as u32);
            let picked_flag_char: char = picked_vec[char_id as usize];
            str_pwd.push(picked_flag_char);
        }
        str_pwd
    }

    /// Generates a number of passwords thanks to `pwd_number` parameter.
    ///
    /// If you want to generate a single password, please use [`Curator::gen_pwd`] instead.
    /// [`Curator::gen_pwd`]: ./struct.Curator.html#method.gen_pwd
    pub fn gen_all_pwd(&mut self, occurrences: usize, pwd_number: usize) -> Vec<String> {
        let mut passwords_list: Vec<String> = Vec::with_capacity(pwd_number);
        for _ in 0..pwd_number {
            passwords_list.push(self.gen_pwd(occurrences));
        }
        passwords_list
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
