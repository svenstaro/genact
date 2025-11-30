//! Pretend to do a full WPT run
use async_trait::async_trait;
use rand::prelude::IndexedRandom;
use rand::{Rng, rng};

use crate::SPEED_FACTOR;
use crate::args::AppConfig;
use crate::data::WPT_TESTS_LIST;
use crate::io::{csleep, newline, print};
use crate::modules::Module;

pub struct WPT;

#[async_trait(?Send)]
impl Module for WPT {
    fn name(&self) -> &'static str {
        "wpt"
    }

    fn signature(&self) -> String {
        "wpt run -f --browser-version=\"1.0\" --processes=16".to_string()
    }

    async fn run(&self, _appconfig: &AppConfig) {
        let mut rng = rng();

        // first: "Running %d tests in web-platform-tests"; pick between ~64k and ~65k.
        let num_tests = rng.random_range(64000..65000);
        print(format!(
            "Running {num_tests} tests in web-platform-tests\r\n"
        ))
        .await;

        let speed_factor = *SPEED_FACTOR.lock().await;

        csleep((2000.0 / speed_factor) as u64).await;

        let mut tests_run = 0;
        let mut failed_subtests = 0;
        let mut all_subtests = 0;
        let mut last_test = &"";
        let mut last_run_tests = 0;

        const PREFIX: &str = "\x1b[A\x1b[K";
        const SUBTEST_RESULT_PREFIX_START: &str = "    ▶ ";
        const SUBTEST_RESULT_PREFIX_MIDDLE: &str = "    │ ";
        const SUBTEST_RESULT_PREFIX_END: &str = "    └ ";

        while tests_run < num_tests {
            if last_run_tests > 0 {
                let subtests_to_run = rng.random_range(0..64) * last_run_tests;
                all_subtests += subtests_to_run;
                for _ in 0..subtests_to_run {
                    // This roughly gives the same failure rate as chrome on WPT.
                    if rng.random_bool(0.03) {
                        // Fail this test.
                        let expected = &["PASS", "FAIL"].choose(&mut rng).unwrap();
                        let is_expected_fail = (*expected) == &"FAIL";
                        let actual = if is_expected_fail {
                            &["PASS", "TIMEOUT"]
                        } else {
                            &["FAIL", "TIMEOUT"]
                        }
                        .choose(&mut rng)
                        .unwrap();
                        let is_timeout = actual == &"TIMEOUT";
                        // extract from last_test some sensible test name
                        let test_name = last_test
                            .split('/')
                            .next_back()
                            .unwrap_or("assert")
                            .split('.')
                            .next()
                            .unwrap_or("assert");
                        // make up some fun assertion
                        let random_assertion = if is_expected_fail {
                            "".into()
                        } else {
                            match rng.random_range(0..5) {
                                0 => format!("approx_equals: expected {} +/- {} but got {}",
                                    rng.random_range(1..100), rng.random_range(1..10), rng.random_range(1..100)),
                                1 => format!("array_equals: expected property {} to be {} but got {}",
                                    rng.random_range(1..10), rng.random_range(1..10), rng.random_range(1..10)),
                                2 => format!("equals: expected {} but got {}",
                                    rng.random_range(1..1000), rng.random_range(1..1000)),
                                3 => "throws_dom: function \"() => test()\" threw object \"TypeError: undefined is not a function\" that is not a DOMException".into(),
                                _ => "true: expected true but got false".into(),
                            }
                        };

                        if is_timeout {
                            print(format!(
                                "{}{}Timed out in {}",
                                PREFIX, SUBTEST_RESULT_PREFIX_START, last_test
                            ))
                            .await;
                            newline().await;
                        } else {
                            print(format!(
                                "{}{}Unexpected subtest result in {}:",
                                PREFIX, SUBTEST_RESULT_PREFIX_START, last_test
                            ))
                            .await;
                            newline().await;
                        }

                        print(format!(
                            "{}ERROR [expected {}] {} test_{} assert_{}",
                            SUBTEST_RESULT_PREFIX_MIDDLE,
                            expected,
                            actual,
                            test_name,
                            random_assertion
                        ))
                        .await;

                        newline().await;

                        if !is_timeout {
                            // make a fake stack trace
                            print(format!("{}  at <unknown>", SUBTEST_RESULT_PREFIX_MIDDLE)).await;
                            newline().await;
                            print(format!("{}  at assert_wrapper (http://web-platform.test:8000/resources/testharness.js:1467:35)", SUBTEST_RESULT_PREFIX_MIDDLE)).await;
                            newline().await;
                            // at _assert (http://web-platform.test:8000/<test_path>:<line>:<col>)
                            let line = rng.random_range(10..500);
                            let col = rng.random_range(10..80);
                            print(format!(
                                "{}  at _assert (http://web-platform.test:8000/{}:{}:{})",
                                SUBTEST_RESULT_PREFIX_MIDDLE, last_test, line, col
                            ))
                            .await;
                            newline().await;
                            // at http://web-platform.test:8000/<test_path>:<line>:<col>
                            let line = rng.random_range(10..50);
                            let col = rng.random_range(10..80);
                            print(format!(
                                "{}  at http://web-platform.test:8000/{}:{}:{}",
                                SUBTEST_RESULT_PREFIX_END, last_test, line, col
                            ))
                            .await;
                            newline().await;
                            newline().await;
                        }

                        failed_subtests += 1;
                    }
                }
            }

            // Grab a random number of tests to run (0..16).
            let tests_this_round = rng.random_range(0..16);
            let tests_ran_so_far = tests_run;
            tests_run += tests_this_round;

            if last_run_tests == 0 {
                print(format!(
                    "{}[{}/{}] No tests running.",
                    PREFIX, tests_ran_so_far, num_tests
                ))
                .await;
                newline().await;
            } else {
                print(format!("{}[{}/{}] ", PREFIX, tests_ran_so_far, num_tests)).await;
                // Grab some random test names.
                for _ in 0..tests_this_round {
                    last_test = WPT_TESTS_LIST.choose(&mut rng).unwrap();
                    print(format!("{} ", last_test)).await;
                }
                newline().await;
            }

            last_run_tests = tests_this_round;

            let sleep_time = rng.random_range(20..60) * tests_this_round as u64;
            csleep((sleep_time as f32 / speed_factor) as u64).await;
        }

        print(format!(
            "\x1b[A\x1b[KRan {}/{} tests with {} failed subtests of {}.\r\n",
            tests_run, num_tests, failed_subtests, all_subtests
        ))
        .await;
    }
}
