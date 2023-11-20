extern crate chrono;
extern crate daemonize;
extern crate system_shutdown;

use chrono::Timelike;
use daemonize::Daemonize;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let stdout = File::create("stdout.log").expect("Could not open stdout.log!");
    let stderr = File::create("stderr.log").expect("Could not open stderr.log!");

    let daemon = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(daemon);

    match daemon.start() {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}

fn daemon() {
    let min_h: u8 = 8;
    let max_h: u8 = 22;

    loop {
        let current_time = chrono::Local::now().hour();

        if current_time >= max_h as u32 || current_time <= min_h as u32 {
            system_shutdown::force_logout().expect("Could not logout!");
        }

        thread::sleep(Duration::from_secs(60));
    }
}
