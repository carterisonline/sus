use fake_tty::bash_command;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub fn ping(addrs: &Vec<String>) -> bool {
    let mut fine = true;
    for addr in addrs {
        let done = Arc::new(RwLock::new(false));
        let pb = ProgressBar::new(2000);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    format!(
                        "{{spinner:.green}} {{elapsed}} [{{wide_bar:.cyan/blue}}] ... {}",
                        addr
                    )
                    .as_str(),
                )
                .progress_chars("->-"),
        );

        let dclone = Arc::clone(&done);

        thread::spawn(move || {
            for _ in 0..2000 {
                pb.inc(1);
                thread::sleep(Duration::from_millis(10));
                if *(dclone.read().unwrap()) == true {
                    break;
                }
            }
        });

        let output = bash_command(format!("ping {} -c 3 -w 20", addr).as_str())
            .output()
            .unwrap();

        let mut w = done.write().unwrap();
        *w = true;

        if !output.status.success() {
            fine = false
        }
    }

    fine
}
