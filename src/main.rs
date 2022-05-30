mod tun;
mod ifstructs;
mod args;
mod tunnel;
mod ping;
mod routes;

use clap::Parser;

fn try_main() -> Result<(), String> {
    let args = args::Args::parse();

    routes::enable_ip_forward(true)?;

    let tun_fd = tun::create_tun_interface(&args.iface_name, args.internal_ip, args.internal_netmask, args.mtu)?;

    if args.server {
        tunnel::start_server(tun_fd, args.mtu)?;
    } else {
        let remote = args.remote_ip.ok_or_else(|| format!("No remote ip but one expected"))?;
        let out_iface_name = args.out_iface_name.ok_or_else(|| format!("No output interface name but one expected"))?;

        routes::create_route(&args.iface_name, &out_iface_name, &remote)?;
        println!("Routes created");

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
