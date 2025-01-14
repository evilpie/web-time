//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, SystemTime};

use self::util::{sleep, DIFF};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// [`SystemTime::add_assign()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
#[should_panic = "overflow when adding duration to instant"]
async fn add_assign_failure() {
	sleep(DIFF).await;
	let mut time = SystemTime::now();
	time += Duration::MAX;
}
