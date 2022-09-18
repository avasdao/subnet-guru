use std::time::{SystemTime, UNIX_EPOCH};

/**
 * Seconds
 * 
 * Returns the number of seconds since the epoch.
 */
pub fn seconds() -> u64 {
    /* Calculate time since the epoch. */
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Oops! Time went backwards!");

    since_the_epoch.as_secs()
}

/**
 * Milliseconds
 * 
 * Returns the number of milliseconds since the epoch.
 */
pub fn milliseconds() -> u64 {
    /* Calculate time since the epoch. */
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Oops! Time went backwards!");

    since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

/* Alias for seconds. */
pub fn now() -> u64 {
    seconds()
}

/* Alias for milliseconds. */
pub fn ms() -> u64 {
    milliseconds()
}

/* Alias for milliseconds. */
pub fn millis() -> u64 {
    milliseconds()
}
