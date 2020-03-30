
extern crate rand;

use std::{
    env,
    process::{
        exit,
        Command,
    },
    fs,
    str::FromStr,
    fmt::Display,
    path::{PathBuf, Path},
    time::Duration,
    thread::sleep,
};

pub fn envvar_or<T, F>(var: &str, default: F) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Display,
    F: FnOnce() -> T,
{
    match env::var(var).ok() {
        Some(val) => match T::from_str(&val) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("error parsing env var {:?}:\n{}", var, e);
                exit(1)
            },
        },
        None => {
            default()
        }
    }
}

pub fn rand_hex(len: usize) -> String {
    let mut buf = String::with_capacity(len);
    for _ in 0..len {
        let nibble = rand::random::<u8>() & 0xF;
        let digit = match nibble{
            0x0..=0x9 => char::from(nibble + b'0'),
            0xA..=0xF => char::from(nibble - 0xA + b'A'),
            _ => unreachable!()
        };
        buf.push(digit);
    }
    buf
}

pub fn exec(cmd: &str) -> Result<(), ()> {
    let mut terms = cmd.split_whitespace();
    let cmd = terms.next()
        .ok_or_else(|| 
            eprintln!("[ERROR] attempt to execute empty command"))?;
    let status = Command::new(cmd)
        .args(terms)
        .status()
        .map_err(|e| 
            eprintln!("[ERROR] failure to execute command: {}", e))?;
    if status.success() {
        Ok({})
    } else {
        eprintln!("[ERROR] process returned failure code: {:?}", status);
        Err({})
    }
}

macro_rules! exec {
    ($($t:tt)*)=>{ $crate::exec(format!($($t)*).as_str()) };
}

pub fn write_file<P, C>(path: P, contents: C)
where
    P: AsRef<Path>, 
    C: AsRef<[u8]>,
{
    if let Err(e) = fs::write(path.as_ref(), contents) 
    {
        eprintln!("[ERROR] failure to write file at {:?}:\n{}", 
            path.as_ref(),
            e);
        exit(1);
    }
}

fn main() {
    let restic_db = envvar_or(
        "RESTIC_DB", 
        || PathBuf::from("/resticdb"));
    let restic_pass = envvar_or(
        "RESTIC_PASS", 
        || PathBuf::from("/resticpass/password"));

    // create the DB if it's never beenc reated before
    if !(
        restic_pass.exists() && 
        exec!("restic stats --password-file {:?} -r {:?}", restic_pass, restic_db)
            .is_ok()
    ) {
        println!("[INFO] creating new restic database for backups");

        // generate password
        if !restic_pass.exists() {
            write_file(&restic_pass, rand_hex(32));
        }

        // init database
        if exec!(
            "restic init --repo {:?} --password-file {:?}",
            restic_db,
            restic_pass,
        ).is_err() {
            exit(1);
        } 
    }

    let seconds = envvar_or("SECONDS", || 20 * 60);
    println!("making a backup every {} seconds", seconds);
    let duration = Duration::from_secs(seconds);

    loop {
        println!("==== MAKING A BACKUP ====");
        let result = Ok({})
            .and_then(|()| exec("./mcrcon -p password 'say now creating server backup.'"))
            .and_then(|()| exec("./mcrcon -p password 'save-off'"))
            .and_then(|()| {
                println!("backing up server");
                exec!(
                    "restic -r {:?} --password-file {:?} backup --exclude /mcserver/logs /mcserver",
                    restic_db,
                    restic_pass)
            })
            .and_then(|()| exec("./mcrcon -p password 'save-on'"))
            .and_then(|()| exec("./mcrcon -p password 'say server backup complete.'"));
        if result.is_err() {
            println!("[WARN] server backup error");
            let _ = exec("./mcrcon -p password 'say [WARN] server backup error.'");
            let _ = exec("./mcrcon -p password 'save-on'");
        }

        sleep(duration);
    }

}