mod tun;
mod ifstructs;
mod args;
mod tunnel;
mod ping;
mod routes;

use clap::Parser;

fn try_main() -> Result<(), String> {
    let args = args::Args::parse();


    let tun_fd = tun::create_tun_interface(&args.iface_name, args.internal_ip, args.internal_netmask, args.mtu)?;


    if args.server {
        // tunnel::start_server()?;
    } else {
        let remote = args.remote_ip.ok_or_else(|| format!("No remote ip but one expected"))?;

        routes::create_route(&args.iface_name, &remote)?;
        println!("Route created");

        tunnel::start_client(tun_fd, args.mtu, remote)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }  
}
