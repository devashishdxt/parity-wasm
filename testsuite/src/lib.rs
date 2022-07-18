#![cfg(test)]

use std::{ffi::OsStr, path::Path};

mod run;

const BASIC_BLACKLIST: [&str; 2] = [
	// those use unsupported i32_trunc_sat_* instructions
	"binary-leb128.wast",
	"conversions.wast",
];

#[test_generator::test_resources("testsuite/spec/*.wast")]
fn basic(path: &str) {
	let path = Path::new(path);
	let blacklisted = path
		.file_name()
		.map(|file| BASIC_BLACKLIST.iter().any(|black| OsStr::new(black) == file))
		.unwrap_or(false);

	if !blacklisted {
		run::check(path);
	}
}

#[test_generator::test_resources("testsuite/spec/proposals/threads/*.wast")]
fn threads(path: &str) {
	run::check(Path::new(path));
}
