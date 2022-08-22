
use std::env; 
use std::net::IpAddr;
 
std::str::FromStr;
 // allows string conversion from string to IP type


struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

// instantiate struct
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(IpAddr) = IpAddr::from_str(&f) {
            
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

