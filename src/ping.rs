use std::str::FromStr;
use std::{ffi::c_void, net::Ipv4Addr};
use std::{mem, net};
use windows::core::{Error, Result};
use windows::Win32::Foundation::{GetLastError, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::NetworkManagement::IpHelper::{
    IcmpCloseHandle, IcmpCreateFile, IcmpSendEcho2Ex,
};
use windows::Win32::NetworkManagement::IpHelper::{ICMP_ECHO_REPLY, IP_OPTION_INFORMATION};

use crate::cli_lib::PingArgs;
use crate::logger::logger::Logger;
pub fn ping(args: &PingArgs) -> Result<()> {
    let logger = Logger::create_instance(&args.verbosity);
    logger.info(&format!("Pinging: {}, TTL: {}", args.dest, args.ttl));
    let _ = call_icmp_echo2_ex(args, &logger);

    Ok(())
}

fn parse_dns_name_or_ip_into_ipv4_ip(dns_or_ip_string: &str, logger: &Logger) -> String {
    // Assume that this is an ip address. Let it error out in case of a DNS name.
    let (resolved_ip, dns_name) = match net::IpAddr::from_str(dns_or_ip_string) {
        Ok(ip) => {
            // This is an IP address, let's lookup it's dns name.
            logger.debug(&format!("Resolving '{}'", ip));
            let dns_name = match dns_lookup::lookup_addr(&ip) {
                Ok(name) => {
                    logger.debug(&format!("Resolved: '{}'", name));
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
                    logger.error(&format!(
                        "DNS lookup failed for '{}': {}",
                        dns_or_ip_string, err
                    ));
                    panic!("dns lookup failed for '{}'", dns_or_ip_string);
                }
            };
            (ip, dns_or_ip_string.to_string())
        }
    };

    logger.debug(&format!("Resolved IP '{}' for '{}'", resolved_ip, dns_name));

    return resolved_ip.to_string();
}

fn handle_response(result: u32, reply_buf: Vec<u8>, logger: &Logger) {
    logger.debug(&format!("ICMP Response Code: {:#?}", result));

    if result == 0 {
        let error_code: windows::Win32::Foundation::WIN32_ERROR = unsafe { GetLastError() };
        let err = Error::from(error_code);

        logger.error(&format!("{}:{}", error_code.0, err.message()));
    } else {
        let reply: &ICMP_ECHO_REPLY = unsafe { mem::transmute(&reply_buf[0]) };
        let ip_string = Ipv4Addr::from(reply.Address);
        logger.debug(&format!("REPLY: {:#?}", reply));
        logger.info(&format!("SUCCESS: Response from: {}", ip_string));
    }
}

fn call_icmp_echo2_ex(ping_args: &PingArgs, logger: &Logger) -> Result<()> {
    // These constants, when placed outside the function and called via multiple threads, the second thread always fails.
    const PING_PAYLOAD: &str = "muttumuttu";

    let ip_opts: IP_OPTION_INFORMATION = IP_OPTION_INFORMATION {
        Ttl: ping_args.ttl,
        Tos: 0,
        Flags: 0,
        OptionsSize: 0,
        OptionsData: 0 as *mut u8,
    };
    const ICMP_ECHO_REPLY_SIZE: usize = mem::size_of::<ICMP_ECHO_REPLY>();
    const REPLY_BUF_SIZE: usize = ICMP_ECHO_REPLY_SIZE + 8 + PING_PAYLOAD.len();

    // Try below sizes in that order, if error 11010 occurs.
    //const REPLY_BUF_SIZE: usize = ICMP_ECHO_REPLY_SIZE + 8 + PING_PAYLOAD.len() + (8 * 1024);
    //const REPLY_BUF_SIZE: usize = 32 * 1024;

    logger.debug(&format!("REPLY_BUF_SIZE: {}", REPLY_BUF_SIZE));

    const TIME_OUT: u32 = 4000;

    let ip_str = parse_dns_name_or_ip_into_ipv4_ip(&ping_args.dest, &logger);
    let ip: Ipv4Addr = Ipv4Addr::from_str(&ip_str).unwrap();

    // Convert to big-endian explicitly (or else 11010 error occurs).
    let addr_u32: u32 = u32::from(ip).to_be();

    let mut reply_buf = vec![0u8; REPLY_BUF_SIZE];
    let evt: Option<HANDLE> = None;
    logger.debug(&format!("Pinging IP (u32): {}...", addr_u32));

    unsafe {
        let h_icmp: HANDLE = IcmpCreateFile()?;
        assert!(h_icmp != INVALID_HANDLE_VALUE);

        let result = IcmpSendEcho2Ex(
            h_icmp,
            evt,
            None,
            None,
            0u32, // Let the OS choose.
            addr_u32,
            PING_PAYLOAD.as_ptr() as *const c_void,
            PING_PAYLOAD.len() as u16,
            Some(&ip_opts),
            reply_buf.as_mut_ptr() as *mut c_void,
            reply_buf.len() as u32,
            TIME_OUT,
        );

        let _ = IcmpCloseHandle(h_icmp);
        handle_response(result, reply_buf, &logger);
    }

    Ok(())
}

// fn call_icmp_echo(args: &PingArgs) {
//     //let ip: Ipv4Addr = Ipv4Addr::from_str("8.8.8.8").unwrap();
//     let ip_str = parse_dns_name_or_ip_into_ipv4_ip(&args.dest);
//     let ip: Ipv4Addr = Ipv4Addr::from_str(&ip_str).unwrap();

//     let mut reply_buf = vec![0u8; REPLY_BUF_SIZE];
//     let addr_u32: u32 = ip.into();

//     unsafe {
//         /*
//             Calculating the reply size (from MSDN https://learn.microsoft.com/en-us/windows/win32/api/icmpapi/nf-icmpapi-icmpsendecho)
//             "The allocated size, in bytes, of the reply buffer. The buffer should be large enough to hold at least one ICMP_ECHO_REPLY structure plus RequestSize bytes of data.
//             This buffer should also be large enough to also hold 8 more bytes of data (the size of an ICMP error message)."

//         */
//         let handle: windows::core::Result<IcmpHandle> = IcmpCreateFile();

//         let result = IcmpSendEcho::<IcmpHandle>(
//             handle.unwrap(),
//             addr_u32,
//             PING_PAYLOAD.as_ptr() as *const c_void, // request data
//             PING_PAYLOAD.len() as u16,
//             Some(&IP_OPTS),
//             reply_buf.as_mut_ptr() as *mut c_void, // reply buffer
//             REPLY_BUF_SIZE as u32,
//             TIME_OUT, // timeout (4 seconds)
//         );

//         // TODO: Need to figure out how to not use a 'moved' variable ('handle').
//         // IcmpCloseHandle(handle.unwrap());

//         handle_response(result, reply_buf);
//     }
// }
