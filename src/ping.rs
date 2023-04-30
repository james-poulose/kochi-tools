use std::str::FromStr;
use std::{ffi::c_void, net::Ipv4Addr};
use std::{mem, net};
use win32_error::Win32Error;
use windows::imp::GetLastError;
use windows::Win32::Foundation::HANDLE;

use windows::Win32::NetworkManagement::IpHelper::{
    /*IcmpCloseHandle,*/ IcmpCreateFile, IcmpHandle, IcmpSendEcho, IcmpSendEcho2Ex,
};
use windows::Win32::NetworkManagement::IpHelper::{ICMP_ECHO_REPLY, IP_OPTION_INFORMATION};

use crate::cli_lib::{Cli, Commands, PingArgs};

const PING_PAYLOAD: &str = "MuttuMuttu";
const IP_OPTS: IP_OPTION_INFORMATION = IP_OPTION_INFORMATION {
    Ttl: 128,
    Tos: 0,
    Flags: 0,
    OptionsSize: 0,
    OptionsData: 0 as *mut u8,
};
const REPLY_SIZE: usize = mem::size_of::<ICMP_ECHO_REPLY>();
const REPLY_BUF_SIZE: usize = REPLY_SIZE + 8 + PING_PAYLOAD.len();
const TIME_OUT: u32 = 4000;

pub fn ping(cli: &Cli, args: &PingArgs) {
    println!("pinging {}, ttl is: {}", args.dest, args.ttl);

    // ICMPEcho is the oldest version and is added only for testing/diagnostics.
    // call_icmp_echo(args);
    call_icmp_echo2_ex(args);
}

fn parse_dns_name_or_ip_into_ipv4_ip(dns_or_ip_string: &str) -> String {
    // Assume that this is an ip address. Let it error out in case of a DNS name.
    let (resolved_ip, dns_name) = match net::IpAddr::from_str(dns_or_ip_string) {
        Ok(ip) => {
            // This is an IP address, let's lookup it's dns name.
            println!("Resolving '{}'", ip);
            let dns_name = match dns_lookup::lookup_addr(&ip) {
                Ok(name) => {
                    println!("Resolved: '{}'", name);
                    name
                }
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
    println!("Pinging {}", dns_name);

    return resolved_ip.to_string();
}

fn call_icmp_echo(args: &PingArgs) {
    //let ip: Ipv4Addr = Ipv4Addr::from_str("8.8.8.8").unwrap();
    let ip_str = parse_dns_name_or_ip_into_ipv4_ip(&args.dest);
    let ip: Ipv4Addr = Ipv4Addr::from_str(&ip_str).unwrap();

    let mut reply_buf = vec![0u8; REPLY_BUF_SIZE];
    let addr_u32: u32 = ip.into();

    unsafe {
        /*
            Calculating the reply size (from MSDN https://learn.microsoft.com/en-us/windows/win32/api/icmpapi/nf-icmpapi-icmpsendecho)
            "The allocated size, in bytes, of the reply buffer. The buffer should be large enough to hold at least one ICMP_ECHO_REPLY structure plus RequestSize bytes of data.
            This buffer should also be large enough to also hold 8 more bytes of data (the size of an ICMP error message)."

        */

        let handle: windows::core::Result<IcmpHandle> = IcmpCreateFile();

        let result = IcmpSendEcho::<IcmpHandle>(
            handle.unwrap(),
            addr_u32,
            PING_PAYLOAD.as_ptr() as *const c_void, // request data
            PING_PAYLOAD.len() as u16,
            Some(&IP_OPTS),
            reply_buf.as_mut_ptr() as *mut c_void, // reply buffer
            REPLY_BUF_SIZE as u32,
            TIME_OUT, // timeout (4 seconds)
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
        let ip_string = Ipv4Addr::from(repl.Address);
        println!("Response from: {}", ip_string);
    }
}

fn call_icmp_echo2_ex(args: &PingArgs) {
    let ip_str = parse_dns_name_or_ip_into_ipv4_ip(&args.dest);
    let ip: Ipv4Addr = Ipv4Addr::from_str(&ip_str).unwrap();
    //let ip: Ipv4Addr = Ipv4Addr::from_str("8.8.8.8").unwrap();
    let addr_u32: u32 = ip.into();
    let mut reply_buf = vec![0u8; REPLY_BUF_SIZE];
    let evt: HANDLE = HANDLE(0); // Trying 0 instead of NULL

    unsafe {
        let handle: windows::core::Result<IcmpHandle> = IcmpCreateFile();
        let result = IcmpSendEcho2Ex::<IcmpHandle, HANDLE>(
            handle.unwrap(),
            evt,
            None,
            None,
            0,
            addr_u32,
            PING_PAYLOAD.as_ptr() as *const c_void,
            PING_PAYLOAD.len() as u16,
            Some(&IP_OPTS),
            reply_buf.as_mut_ptr() as *mut c_void,
            REPLY_BUF_SIZE as u32,
            TIME_OUT,
        );

        // TODO: Usage of moved variable error.
        //windows::Win32::NetworkManagement::IpHelper::IcmpCloseHandle(handle.unwrap());
        handle_response(result, reply_buf);
    }
}
