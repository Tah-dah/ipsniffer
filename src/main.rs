
use std::env; 
use std::net::IpAddr;
use std::str::FromStr;
 // allows string conversion from string to IP type


struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

// instantiate struct
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 { // program name, thread flag
            return Err("Not enough arguments");
        } else if args.len() > 4 { 
            return Err("Too many arguments");
        }

        // vairable looks at first index of argument
        let f = args[1].clone();
        // destruct IP fromstr returns result
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!( "-j to select thread amount
                \r\n -h or -help to show help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments"); 
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){ // args index 3 top ip add
                    Ok(s) => s,
                    Err(_) => return Err("Not valid IPADDR, must be V4 or V6")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("Ivalid arguments")
            }
        }
    }
}

fn main() {
    //arguments passed into vector of strings
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    

    }



// ipsniffer.exe -h //flag present help screen
// ipsniffer.exe -j 100 192.168.1.1 // set threads
// ipsniffer.exe 192.168.1.1 //calling tool on IP add will use threads specified

