use std::str::FromStr;
use std::{ffi::c_void, net::Ipv4Addr};
use std::{mem, net};
use win32_error::Win32Error;
use windows::imp::GetLastError;

use windows::Win32::NetworkManagement::IpHelper::{
    /*IcmpCloseHandle,*/ IcmpCreateFile, IcmpHandle, IcmpSendEcho,
};
use windows::Win32::NetworkManagement::IpHelper::{ICMP_ECHO_REPLY, IP_OPTION_INFORMATION};

use crate::cli_lib::{Cli, Commands, PingArgs};

pub fn ping(cli: Cli) {
    //let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ping(args) => {
            println!("'ping' for {}, ttl is: {}", args.dest, args.ttl);
            call_icmp_echo(args);
        }
        Commands::Trace(args) => {
            println!("'trace' for {}, ttl is: {}", args.dest, args.ttl);
        }
    }
}

fn parse_dns_name_or_ip_into_ipv4_ip(dns_or_ip_string: &str) -> String {
    // Assume that this is an ip address. Let it error out in case of a DNS name.
    let (resolved_ip, dns_name) = match net::IpAddr::from_str(dns_or_ip_string) {
        Ok(ip) => {
            // This is an IP address, let's lookup it's dns name.
            let dns_name = match dns_lookup::lookup_addr(&ip) {
                Ok(name) => name,
                // There is no point in handling error here, just return the IP.
                Err(_) => dns_or_ip_string.to_string(),
            };
            (ip, dns_name)
        }
        Err(_) => {
            // This is now either a valid DNS name or an invalid IP address or DNS name.
            let ip = match dns_lookup::lookup_host(dns_or_ip_string) {
                Ok(vec_ip) => {
                    let p = vec_ip.into_iter().find(|&ip| ip.is_ipv4()).unwrap();
                    p
                }
                Err(err) => {
                    println!("{}", err);
                    panic!("dns lookup failed for '{}'", dns_or_ip_string);
                }
            };
            (ip, dns_or_ip_string.to_string())
        }
    };
    println!("Mutting {}", dns_name);

    return resolved_ip.to_string();
}

fn call_icmp_echo(args: &PingArgs) {
    //let ip: Ipv4Addr = Ipv4Addr::from_str("8.8.8.8").unwrap();
    let ip_str = parse_dns_name_or_ip_into_ipv4_ip(&args.dest);
    let ip: Ipv4Addr = Ipv4Addr::from_str(&ip_str).unwrap();

    unsafe {
        let addr_u32: u32 = ip.into();

        /*
            Calculating the reply size (from MSDN https://learn.microsoft.com/en-us/windows/win32/api/icmpapi/nf-icmpapi-icmpsendecho)
            "The allocated size, in bytes, of the reply buffer. The buffer should be large enough to hold at least one ICMP_ECHO_REPLY structure plus RequestSize bytes of data.
            This buffer should also be large enough to also hold 8 more bytes of data (the size of an ICMP error message)."

        */
        let payload = "muttumuttu";
        let reply_size = mem::size_of::<ICMP_ECHO_REPLY>();
        let reply_buf_size = reply_size + 8 + payload.len();
        let mut reply_buf = vec![0u8; reply_buf_size];

        let ip_opts = IP_OPTION_INFORMATION {
            Ttl: 128,
            Tos: 0,
            Flags: 0,
            OptionsSize: 0,
            OptionsData: 0 as *mut u8,
        };

        let handle: windows::core::Result<IcmpHandle> = IcmpCreateFile();

        let result = IcmpSendEcho::<IcmpHandle>(
            handle.unwrap(),
            addr_u32,
            payload.as_ptr() as *const c_void, // request data
            payload.len() as u16,
            Some(&ip_opts),
            reply_buf.as_mut_ptr() as *mut c_void, // reply buffer
            reply_buf_size as u32,
            4000, // timeout (4 seconds)
        );

        // TODO: Need to figure out how to not use a 'moved' variable ('handle').
        // IcmpCloseHandle(handle.unwrap());

        handle_response(result, reply_buf);
    }
}

fn handle_response(result: u32, reply_buf: Vec<u8>) {
    println!("Muttu Response: {}", result);

    if result == 0 {
        let error_code = unsafe { GetLastError() };
        let err = Win32Error::from(error_code);
        println!("{}", err.to_string());
    } else {
        let repl: &ICMP_ECHO_REPLY = unsafe { mem::transmute(&reply_buf[0]) };
        println!("Reply:{:#?}", *repl);
    }
}
