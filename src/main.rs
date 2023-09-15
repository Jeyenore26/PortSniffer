use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::{env, io, process, thread, vec};
struct Argument {
    _flag: String,
    ip_addr: IpAddr,
    threads: u16,
}
const MAX: u16 = 65535;
impl Argument {
    fn new(args: &[String]) -> Result<Argument, &'static str> {
        if args.len() < 2 {
            return Err("Missing Arguments");
        } else if args.len() > 4 {
            return Err("Alot of Arguments");
        }
        let f = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            return Ok(Argument {
                _flag: String::from(""),
                ip_addr,
                threads: 4,
            });
        } else {
            let _flag = args[1].clone();
            if _flag.contains("-h") || _flag.contains("-help") && args.len() == 2 {
                println!("========================================\nUsage -j to select how many threads you want /r/n cargo run -- -j no.threads IDaddr /r/n-h or -help to show this message\n========================================");
                return Err("help");
            } else if _flag.contains("-h") || _flag.contains("-help") {
                return Err("Too many arguments");
            } else if _flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR ; Must be IPV4 or IPV6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number {threads}"),
                };
                return Ok(Argument {
                    _flag,
                    ip_addr,
                    threads,
                });
            } else {
                return Err("Invalid syntax");
            }
        };
    }
}
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, numthreads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port);
            }
            Err(_) => {}
        }
        if (MAX - port) <= numthreads {
            break;
        }
        port += numthreads;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let argumnnt = Argument::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments : {}", program, err);
            process::exit(0);
        }
    });
    let threadnum = argumnnt.threads;
    let _addr = argumnnt.ip_addr;
    let (tx, rx) = channel();
    for i in 0..threadnum {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, argumnnt.ip_addr, threadnum);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} IS OPEN", v);
    }
}
