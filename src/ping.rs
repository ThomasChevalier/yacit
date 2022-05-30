use socket2::{Domain, Protocol, Socket, Type, SockAddr};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::mem::MaybeUninit;

use pnet::util::checksum;


fn create_socket() -> Socket{
	let s = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).unwrap();
	return s;
}

pub fn create_socket_client(address: Ipv4Addr) -> Result<Socket, String>{
	let s  = create_socket();
	let addr = SockAddr::from(SocketAddr::new(IpAddr::V4(address), 0));
	s.connect(&addr).map_err(|e| format!("Cannot connect socket to {}: {}", address, e))?;
	Ok(s)
}

pub fn create_socket_server() -> Result<Socket, String>{
	let s  = create_socket();
	let addr = SockAddr::from(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0));
	s.bind(&addr).map_err(|e| format!("Cannot bind socket to 0.0.0.0: {}", e))?;
	Ok(s)
}


pub struct IcmpV4 {
    pub payload : Vec<u8>,
    type_ : IcmpType,
    code : u8,
}

impl IcmpV4 {


	pub fn create_icmp(type_: IcmpType, code: u8, payload : Vec<u8>) -> IcmpV4 {
		let icmp = IcmpV4 {payload: payload,type_: type_,code: code};
		return icmp;
	}

	pub fn send_ping(&self, soc : &Socket) -> Result<(), String> {
		let mut paquet = vec![self.type_.to_byte(), self.code,0,0,0,0,0,0];
		paquet.extend(&self.payload);

		let [u1,u2] = calcul_checksum(&paquet).to_be_bytes();
		paquet[2] = u1;
		paquet[3] = u2;
		
		soc.send(&paquet).map_err(|err| format!("send_ping error: {}", err))?;
		Ok(())
	}

	pub fn recv_ping(soc : &Socket, mtu: i32) -> (IcmpV4,SockAddr) {
		let mut buffer: Vec<MaybeUninit<u8>> = vec![MaybeUninit::<u8>::uninit(); mtu as usize];
		
		let (size,addr_rcv) = soc.recv_from(buffer.as_mut_slice()).unwrap();

		let mut data = Vec::new();
		
		for i in 0..size {
    		data.push(unsafe { buffer[i].assume_init()})
		}
		return (IcmpV4::parse_icmp(data),addr_rcv);
	}

	fn parse_icmp(mut data : Vec<u8>) -> IcmpV4 {
		let type_ = IcmpType::from_byte(data[20]).unwrap();
		let code = data[21];
		let payload = data.split_off(28);
		return IcmpV4::create_icmp(type_,code,payload);

	}

	pub fn is_request(&self) -> bool {
		match self.type_ {
			IcmpType::EchoRequest => true,
			_ => false
		}
	}
}
impl ToString for IcmpV4{

	fn to_string(&self) -> String {
		format!("IcmpV4 :\n  - type {:?}\n  - code {}\n  - payload {:?}",self.type_.to_string(),self.code,self.payload)
	}
}


fn calcul_checksum(data : &[u8]) -> u16{
	/*
	let mut res : u16 = 0;
	for i in 0..data.len()/2{
		let tmp : u64 = (res as u64) +(((data[2*i] as u16) <<8)|(data[2*i+1] as u16)) as u64;
		res = tmp as u16;
	}
	if data.len() % 2 == 1 {
		let tmp : u64 = (res as u64) +(((data[data.len()-1] as u16) <<8) as u64);
		res = tmp as u16;
	}
	return !res;
	*/
	return checksum(data,1)
}


// IcmpType 

pub enum IcmpType {
	EchoRequest,
	EchoReply,
}

impl IcmpType {
	pub fn to_byte(&self) -> u8 {
		match self {
			IcmpType::EchoReply => return 0,
			IcmpType::EchoRequest => return 8,
		}
	}

	pub fn from_byte(value : u8) -> Result<IcmpType, &'static str> {
		match value {
			8 => Ok(IcmpType::EchoRequest),
			0 => Ok(IcmpType::EchoReply),
			_ => Err("Invalid Icmp type value."),
		}
	}

}

impl ToString for IcmpType {
	fn to_string(&self) -> String {
		match self {
			IcmpType::EchoReply => "EchoReply".to_string(),
			IcmpType::EchoRequest => "EchoRequest".to_string(),
		}
	}
}