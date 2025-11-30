//! Pretend to do a full WPT run
use async_trait::async_trait;
use rand::prelude::IndexedRandom;
use rand::{Rng, rng};

use crate::args::AppConfig;
use crate::data::{CSS_PROPERTIES_LIST, WEB_APIS_LIST, WPT_CATEGORIES_LIST};
use crate::io::{csleep, newline, print};
use crate::modules::Module;

const ENCODING_TYPES: &[&str] = &[
    "legacy-mb-japanese",
    "legacy-mb-korean",
    "legacy-mb-schinese",
    "legacy-mb-tchinese",
];

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

        // first: "Running %d tests in web-platform-tests"; between ~64k and ~65k is the "real" range.
        let num_tests = rng.random_range(5000..30000);
        print(format!(
            "Running {num_tests} tests in web-platform-tests\r\n"
        ))
        .await;

        csleep(2000).await;

        let mut tests_run = 0;
        let mut failed_subtests = 0;
        let mut all_subtests = 0;
        let mut last_test = String::new();
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
                            // make a fake stack trace with random number of frames
                            let stack_trace = generate_stack_trace(
                                &mut rng,
                                &last_test,
                                SUBTEST_RESULT_PREFIX_MIDDLE,
                                SUBTEST_RESULT_PREFIX_END,
                            );
                            print(stack_trace).await;
                            newline().await;
                        }

                        failed_subtests += 1;
                    }
                }
            }

            // Grab a random number of tests to run.
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
                // Make some random test names.
                for _ in 0..tests_this_round {
                    last_test = generate_wpt_test_path(&mut rng);
                    print(format!("{} ", last_test)).await;
                }
                newline().await;
            }

            last_run_tests = tests_this_round;

            csleep(rng.random_range(20..60) * tests_this_round as u64).await;
        }

        print(format!(
            "\x1b[A\x1b[KRan {}/{} tests with {} failed subtests of {}.\r\n",
            tests_run, num_tests, failed_subtests, all_subtests
        ))
        .await;
    }
}

/// Generates a random WPT test path, examples:
/// - css tests: /css/css-color/test-123.html
/// - html tests: /html/push-api/test-45.html
/// - encoding tests: /encoding/legacy-mb-japanese/eucjp-decode-legacy-mb-japanese-1000.html?1000-1999
/// - wpt-specific categories: /webrtc/protocol/test-67.html
/// - other web APIs: /performance-timeline/test-234.html
fn generate_wpt_test_path(rng: &mut rand::rngs::ThreadRng) -> String {
    let choice = rng.random_range(0..100);

    match choice {
        0..=47 => {
            // ~48% css tests
            let prop = CSS_PROPERTIES_LIST.choose(rng).unwrap();
            let test_num = rng.random_range(1..500);
            format!("/css/css-{}/test-{}.html", prop, test_num)
        }
        48..=61 => {
            // ~14% html tests
            let api = WEB_APIS_LIST.choose(rng).unwrap();
            let test_num = rng.random_range(1..300);
            format!("/html/{}/test-{}.html", api, test_num)
        }
        62..=63 => {
            // ~2% encoding tests
            let encoding_type = ENCODING_TYPES.choose(rng).unwrap();
            let test_names = [
                "eucjp-decode",
                "eucjp-encode-form",
                "big5-decode",
                "shift-jis-decode",
            ];
            let test_name = test_names.choose(rng).unwrap();
            let range = rng.random_range(1..20);
            let range_start = range * 1000;
            let range_end = range_start + 999;
            format!(
                "/encoding/{}/{}-{}.html?{}-{}",
                encoding_type, test_name, encoding_type, range_start, range_end
            )
        }
        _ => {
            // ~36% some random web API
            if rng.random_bool(0.5) {
                let category = WPT_CATEGORIES_LIST.choose(rng).unwrap();
                let test_num = rng.random_range(1..200);
                format!("{}/test-{}.html", category, test_num)
            } else {
                let api = WEB_APIS_LIST.choose(rng).unwrap();
                let test_num = rng.random_range(1..500);
                format!("/{}/test-{}.html", api, test_num)
            }
        }
    }
}

/// Generate a fake stack trace with random frames, examples:
/// - "  at testGetBoundingClientRect (http://web-platform.test:8000/html/push-api/test-45.html:23:15)"
/// - "  at assert_wrapper (http://web-platform.test:8000/resources/testharness.js:633:35)"
/// - "  at <unknown>"
/// - "  at http://web-platform.test:8000/resources/testharness.js:1467:34"
fn generate_stack_trace(
    rng: &mut rand::rngs::ThreadRng,
    test_path: &str,
    prefix_middle: &str,
    prefix_end: &str,
) -> String {
    const TESTHARNESS_LINES: &[u32] = &[633, 1467, 1518, 2869]; // Some commonly seen testharness.js line numbers.
    const TEST_FUNC_NAMES: &[&str] = &[
        "testGetBoundingClientRect",
        "test",
        "assert",
        "verify",
        "check",
    ];

    let num_frames = rng.random_range(3..=8);
    let mut frames = String::new();

    for frame_idx in 0..num_frames {
        let is_last_frame = frame_idx == num_frames - 1;
        let is_first_frame = frame_idx == 0;
        let prefix = if is_last_frame {
            prefix_end
        } else {
            prefix_middle
        };

        let frame_str = if is_last_frame {
            // Last frame should be <unknown> or a test file reference
            if rng.random_bool(0.5) {
                "  at <unknown>".to_string()
            } else {
                let line = rng.random_range(1..150);
                let col = rng.random_range(1..100);
                let named_function = rng.random_range(0..10) > 7;
                if named_function {
                    let func = TEST_FUNC_NAMES.choose(rng).unwrap();
                    format!(
                        "  at {} (http://web-platform.test:8000/{}:{}:{})",
                        func, test_path, line, col
                    )
                } else {
                    format!(
                        "  at http://web-platform.test:8000/{}:{}:{}",
                        test_path, line, col
                    )
                }
            }
        } else if is_first_frame || rng.random_bool(0.6) {
            let frame_type = rng.random_range(0..100);
            match frame_type {
                0..=24 => "  at <unknown>".to_string(),
                25..=54 => {
                    let line = TESTHARNESS_LINES.choose(rng).unwrap();
                    format!(
                        "  at assert_wrapper (http://web-platform.test:8000/resources/testharness.js:{}:35)",
                        line
                    )
                }
                55..=79 => {
                    let line = TESTHARNESS_LINES.choose(rng).unwrap();
                    format!(
                        "  at test (http://web-platform.test:8000/resources/testharness.js:{}:34)",
                        line
                    )
                }
                _ => {
                    let line = TESTHARNESS_LINES.choose(rng).unwrap();
                    let col = rng.random_range(20..50);
                    format!(
                        "  at http://web-platform.test:8000/resources/testharness.js:{}:{}",
                        line, col
                    )
                }
            }
        } else {
            let line = rng.random_range(1..150);
            let col = rng.random_range(1..100);
            let named_function = rng.random_range(0..10) > 7;
            if named_function {
                let func = TEST_FUNC_NAMES.choose(rng).unwrap();
                format!(
                    "  at {} (http://web-platform.test:8000/{}:{}:{})",
                    func, test_path, line, col
                )
            } else {
                format!(
                    "  at http://web-platform.test:8000/{}:{}:{}",
                    test_path, line, col
                )
            }
        };

        frames.push_str(&format!("{}{}\r\n", prefix, frame_str));
    }

    frames
}
