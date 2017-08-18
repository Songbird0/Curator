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
//! Curator is an easy to use password generator.
//!
//! ## Example:
//!
//! ```rust,ignore
//! <one flag example here>
//! <three flags example here>
//! <produces ten passwords here>
//! ```
//! */

extern crate clap;

use clap::{App, Arg, SubCommand, AppSettings};
pub mod generator;

const CURATOR_VERSION : &'static str = "0.1.0";
const AUTHOR : &'static str = "Anthony Defranceschi <chaacygg[at]gmail[dot]com>";
const DESCRIPTION : &'static str = r"
    Curator  Copyright (C) 2017  Anthony Defranceschi

    This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
    This is free software, and you are welcome to redistribute it
    under certain conditions; type `show c' for details.
-----------

Generate some strong passwords.
Using:
curator -i -uc -lc -n 20
Prints: <put a password here>";

fn main() {
    let app = App::new("Curator");
    let matches = app
        .version(CURATOR_VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .settings(&[AppSettings::SubcommandsNegateReqs,
            AppSettings::ColorAlways,
            AppSettings::ArgRequiredElseHelp,
            AppSettings::SubcommandRequiredElseHelp])
        .subcommand(SubCommand::with_name("show")
            .author(AUTHOR)
            .about("Show terms and conditions")
            .arg(Arg::with_name("conditions_parts")
                .help("the license part that you want to read")
                .value_name("[w|c]")
                .required(true)
                .takes_value(true)))
        .arg(Arg::with_name("occurrence")
            .short("n")
            .long("number")
            .help("The occurrences number forming the password(8 by default)")
            .value_name("1, 2, ... n")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("lowercase_letters")
            .short("l")
            .long("lcletters")
            .help("Enables lower case letters generation")
            .takes_value(false))
        .arg(Arg::with_name("uppercase_letters")
            .short("u")
            .long("ucletters")
            .help("Enables upper case letters generation")
            .takes_value(false))
        .arg(Arg::with_name("integer")
            .short("i")
            .long("integer")
            .help("Enables integers generation")
            .takes_value(false))
        .arg(Arg::with_name("special_char")
            .short("s")
            .long("specialchar")
            .help("Enable special characters generation")
            .takes_value(false))
        .get_matches();
    if matches.is_present("occurrence") {
        let lc_is_enabled = matches.is_present("lowercase_letters");
        let uc_is_enabled = matches.is_present("uppercase_letters");
        let integer_is_enabled = matches.is_present("integer");
        if !lc_is_enabled &&
            !uc_is_enabled &&
            !integer_is_enabled {
            eprintln!("No flags were supplied. Please read --help command result.");
        }
    }
    if let Some(ref sub_matches) = matches.subcommand_matches("show") {
        let arg = sub_matches.value_of("conditions_parts").unwrap();
        match arg {
            "w" => {
                println!(r"THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW. EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM “AS IS” WITHOUT WARRANTY OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.");
            },
            "c" => {
                println!(r"
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
                ");
            },
            _ => {
                eprintln!("You've typed: '{}'\nInvalid argument. Please read --help command result.", arg);
            }
        }
    }
}
