use std::{
    env,
    io::{self, Write},
    process,
    str::FromStr,
};

use rand::{RngCore, SeedableRng};
use shishua::ShiShuARng;

fn main() {
    let buffer_size = env::var("BUFFER_SIZE")
        .ok()
        .and_then(|s| usize::from_str(&s).ok())
        .unwrap_or(1 << 7);
    let mut rng = ShiShuARng::from_os_rng();
    let mut buf = vec![0; buffer_size];
    let mut stdout = io::stdout().lock();
    loop {
        rng.fill_bytes(&mut buf);
        let Ok(_) = stdout.write_all(&buf) else {
            process::exit(1);
        };
    }
}
