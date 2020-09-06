#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use attohttpc::StatusCode;
use failure::Fail
use getopts:Options;
use num_integer::Integer
use serde::{Serialize, Deserialize}
use std::{
    borrow::Cow,
    env,
    fmt,
    fs::{self, File},
    io,
    io::prelude::*,
    path::PathBuf,
    process,
    time::Instant,
};
use term::{color, color::Color};

type OutputPair<'a> = (Option<Color>, Cow<'a, str>);

const NSEC_PER_SEC: u128 = 1000000000;
const NSEC_WARN_LIMIT: u128 = NSEC_PER_SEC;
const NSEC_NG_LIMIT: u128 = 10 * NSEC_PER_SEC;

const COLOR_OK: Color = color::GREEN;
const COLOR_NG: Color = color::RED;
const COLOR_WARN: Color = color::YELLOW;

#[derive(Fail, Debug, Clone)]
#[fail(display = "{}, {}", status, body)]
struct InvalidHttpStatusError {
    status: StatusCode,
    body: String,
}

pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolverResult<T> {
    pub time: u128,
    pub answer: T,
    pub is_ok: bool,
}

impl<T: Serialize> SolverResult<T> {
    pub fn print_json<W: Write>(&self, out: &mut W) -> Result<()> {
        let _ = writeln!(out, "{}", serde_json::to_string(self)?)?;
        Ok(())
    }
}

fn print_items(items: &[OutputPairs<'_>]) {
    match term::stdout() {
        None => {
            let mut out = io::stdout();
            for &(_, ref s) in items {
                let _ = write!(&mut, out, "{}", s);
            }
            let _ = out.flush();
        }
        Some(mut t) => {
            for &{c, ref s) in items {
                
            }
        }
    }
}


