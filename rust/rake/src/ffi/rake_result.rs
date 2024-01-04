use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::panic::catch_unwind;

use net_fi::FFIResult;

use crate::utils::OptionExt;

thread_local! {
    static LAST_RESULT: RefCell<Option<LastResult>> = RefCell::new(None);
}

struct LastResult {
    value: RakeResult,
    err: Option<String>,
}

/// Possible foreign function call results
#[non_exhaustive]
#[derive(Debug)]
pub enum RakeResult {
    /// Operation was successful
    Ok,

    /// Backend has terminated normally
    Terminated,

    /// One or more arguments were null
    NullArgument,

    /// One or more arguments had a non-valid value
    InvalidArgument,

    /// An internal error occured
    InternalError,
}

impl FFIResult for RakeResult {

    fn catch(f: impl FnOnce() -> Self + std::panic::UnwindSafe) -> Self {
        LAST_RESULT.with(|last_result| {
            *last_result.borrow_mut() = None;
            
            match catch_unwind(f) {
                // No panics
                Ok(rake_result) => {

                    let extract_err = || { format!("{:?}", rake_result) };
                    
                    // Always set the last result so it matches what's returned.
                    // This `Ok` branch doesn't necessarily mean the result is ok,
                    // only that there wasn't a panic.
                    last_result
                        .borrow_mut()
                        .map_mut(|last_result| {
                            last_result.value = rake_result;
                            last_result.err.or_else_mut(|| Some(extract_err()));
                        })
                        .get_or_insert_with(|| LastResult {
                            value: rake_result,
                            err: extract_err(),
                        })
                        .value
                },
                Err(e) => {
                    let extract_panic =
                        || error::extract_panic(&e)
                               .map(|s| format!("internal panic with '{}'", s));
                        
                    // Set the last error to the panic message if it's not already set
                    last_result
                        .borrow_mut()
                        .map_mut(|last_result| {
                            last_result.err.or_else_mut(extract_panic);
                        })
                        .get_or_insert_with(|| LastResult {
                            value: FlareResult::InternalError,
                            err: extract_panic(),
                        })
                        .value
                }
            }
        })
    }

    fn null_arg_error() -> Self {
        todo!()
    }
}

fn process_success(rake_result: RakeResult) {

}