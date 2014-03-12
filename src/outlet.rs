extern crate collections;
extern crate oblw;
extern crate toml;

use collections::{bitv};

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::udp::UdpSocket;
use std::io::{Listener, Acceptor};
use std::io::fs::File;

use std::str;
use std::os;
use std::num;
use std::libc;
use std::io;


fn main() {
	let args = os::args();
	let mut timer = io::Timer::new().unwrap();
	let (mut c, p) = oblw::spawnBytestream(false);
	let root = toml::parse_from_file("outlets.toml").unwrap();

	let getCode = |x: &str| -> bitv::Bitv {
		let mut q: ~str = ~"db.";
		q.push_str(x);
		let data = root.lookup(q);
		match data {
			Some(data) => {
				let data = data.get_vec().unwrap();
				let mut y: ~[oblw::Run] = ~[];
				data.iter().map(|bit| {
					match bit {
						&toml::PosInt(1) => y.push_all_move( ~[oblw::Run { v: 1, ct: 561}, oblw::Run { v: 0, ct: 187}]) ,
						&toml::PosInt(0) => y.push_all_move( ~[oblw::Run { v: 1, ct: 187}, oblw::Run {v: 0, ct: 561}]) ,
						x => println!("wat. got {:?}, expected 1/0",x)
					}}).last();
				y.push(oblw::Run {v: 0, ct: 5000});
				oblw::v2b(oblw::rld(y))
			},
			None => { oblw::v2b([]) }
		}
	};

	if (args.len() - 1) > 0 {
		let syn = getCode(args[1].slice_from(0));
		for _ in range(0, 7) {c = oblw::sendBitstream(syn.clone(), c); }
	}

	else {
		let mut inn = ~[0u8, ..512];
		let mut sock = UdpSocket::bind(SocketAddr{ip:Ipv4Addr(127,0,0,1), port:9997}).unwrap();
		'rpc: loop {
			let (ct, org) = sock.recvfrom(inn.mut_slice_from(0)).unwrap();
			let nu8 = inn.slice_to(inn.iter().position(|&x| x == 0u8).unwrap());
			let name = str::from_utf8(nu8).unwrap();
			for _ in range(0, 7) {c = oblw::sendBitstream(getCode(name), c); }
		}
	}
	timer.sleep(100);
	unsafe {libc::funcs::c95::stdlib::exit(1);}
}
