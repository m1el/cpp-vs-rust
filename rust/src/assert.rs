// TODO(port-later): Redesign this and document rationale.

use crate::util::c_string::*;

#[macro_export]
macro_rules! qljs_always_assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            $crate::assert::report_assertion_failure_c(
                /*file_name=*/ $crate::qljs_c_string!(file!()),
                /*line=*/ line!(),
                /*message=*/ $crate::qljs_c_string!(stringify!($cond)),
            );
            $crate::qljs_assert_trap!();
        }
    };
}

#[macro_export]
macro_rules! qljs_assert {
    ($cond:expr $(,)?) => {
        #[cfg(debug_assertions)]
        $crate::qljs_always_assert!($cond);
    };
    ($cond:expr, $message:expr $(,)?) => {
        // TODO(strager): Include the message.
        #[cfg(debug_assertions)]
        $crate::qljs_always_assert!($cond);
    };
}

#[macro_export]
macro_rules! qljs_slow_assert {
    ($cond:expr $(,)?) => {
        #[cfg(feature = "qljs_debug")]
        $crate::qljs_always_assert!($cond);
    };
}

#[macro_export]
macro_rules! qljs_assert_trap {
    () => {
        // TODO(port-later): Is panic the right choice here? How heavy-weight is it? Could we
        // do something simpler? Maybe something like core::arch::x86_64::ud2 is better.
        panic!("assertion failed");
    };
}

// NOTE(strager): We prefer raw pointers to reduce code bloat when calling the assertion failure
// function. Perhaps we should look at other solutions, such as std::panic::Location.
pub fn report_assertion_failure_c(file_name: *const u8, line: u32, message: *const u8) {
    unsafe {
        let file_name: &str = read_utf8_c_string(file_name);
        let message: &str = read_utf8_c_string(message);
        report_assertion_failure(file_name, line, message);
    }
}

pub fn report_assertion_failure(file_name: &str, line: u32, message: &str) {
    eprintln!(
        "{file_name}:{line}: internal check failed: {message}\n\
               quick-lint-js crashed. Please report this bug here:\n\
               https://quick-lint-js.com/crash-report/\n"
    );
}
