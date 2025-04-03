use std::{thread, time::Duration};

use dioxus::prelude::*;

#[server]
pub async fn fix_timeout() -> Result<(), ServerFnError> {
    thread::sleep(Duration::from_secs(1));
    Ok(())
}
