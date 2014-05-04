extern crate collections;
extern crate oblw;
extern crate toml;
extern crate libc;

use collections::{bitv};

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::udp::UdpSocket;
use std::io::{Listener, Acceptor};
use std::io::fs::File;
use std::strbuf;
use std::str;
use std::os;
use std::num;
use std::io;

fn main() {
	let args = os::args();
	let mut timer = io::Timer::new().unwrap();
	let root = toml::parse_from_file("outlets.toml").unwrap();
	let (c, p) = oblw::spawnBytestream(false);

	let getCode = |x: &str| -> bitv::Bitv {
		let mut q = strbuf::StrBuf::from_str("db.");
		q.push_str(x);
		let data = root.lookup(q.as_slice());
		match data {
			Some(data) => {
				let data = data.get_vec().unwrap();
				let mut y = vec!();
				for _ in range(0, 5) {
					data.iter().map(|bit| {
						match bit {
							&toml::PosInt(1) => y.push_all_move( vec!(oblw::Run { v: 1, ct: 561}, oblw::Run { v: 0, ct: 187})) ,
							&toml::PosInt(0) => y.push_all_move( vec!(oblw::Run { v: 1, ct: 187}, oblw::Run {v: 0, ct: 561})) ,
							x => println!("wat. got {:?}, expected 1/0",x)
						}}).last();
					y.push(oblw::Run {v: 0, ct: 5000});
				}
				oblw::v2b(oblw::rld(y))
			},
			None => { oblw::v2b(vec!()) }
		}
	};

	if (args.len() - 1) > 0 {
		let syn = getCode(args[1].slice_from(0));
		oblw::sendBitstream(syn.clone(), c.clone());
		timer.sleep(1000);
	}

	else {
		let mut inn = ~[0u8, ..512];
		let mut sock = UdpSocket::bind(SocketAddr{ip:Ipv4Addr(127,0,0,1), port:9997}).unwrap();
		'rpc: loop {
			let (ct, org) = sock.recvfrom(inn.mut_slice_from(0)).unwrap();
			let nu8 = inn.slice_to(inn.iter().position(|&x| x == 0u8).unwrap());
			let name = str::from_utf8(nu8).unwrap();
			let syn = getCode(name);
			oblw::sendBitstream(syn.clone(), c.clone());
			timer.sleep(1000);
		}
	}
	unsafe {libc::funcs::c95::stdlib::exit(1);}
}
