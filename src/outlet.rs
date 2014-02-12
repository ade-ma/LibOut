extern mod extra;
extern mod oblw;

use extra::bitv;

use std::os;
use std::num;
use std::libc;
use std::io;

fn outlet(n: uint) -> bitv::Bitv {
	let data: bitv::Bitv = match n {
		1u => bitv::from_fn(25, |x: uint| [1,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,1,1,0,0,0,0,0,0,0][x] == 1),
		2u => bitv::from_fn(25, |x: uint| [1,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,1,0,0,0,0,0][x] == 1),
		3u => bitv::from_fn(25, |x: uint| [1,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,1,1,0,0,0][x] == 1),
		6u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,1,1,0][x] == 1),
		7u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,1,1,0,0,0][x] == 1),
		8u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,1,1,0,0,0,0,0][x] == 1),
		4u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,0,0,1,0,1,0,1,0,1,0,0,1,1,0,0,0,0,0,0,0][x] == 1),
		5u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,0,0,1,0,1,0,1,0,1,1,1,0,0,0,0,0,0,0,0,0][x] == 1),
		10u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,1,1,1,1,0,0,0][x] == 1),
		11u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,1,1,0,0,1,1,0][x] == 1),
		20u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,1,0,0,0,1,0,1,0,1,0,1,1,1,0,0,1,1,0,0,0][x] == 1),
		21u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,1,0,0,0,1,0,1,0,1,0,1,1,1,0,0,0,0,1,1,0][x] == 1),
		41u => bitv::from_fn(25, |x: uint| [0,0,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0][x] == 1),
		
		_ => bitv::from_bools(~[]),
	};
	
	let mut y: ~[oblw::Run] = ~[];
	for bit in data.iter() {
		match bit {
			true => y.push_all_move( ~[oblw::Run { v: 1, ct: 561}, oblw::Run { v: 0, ct: 187}]) ,
			false => y.push_all_move( ~[oblw::Run { v: 1, ct: 187}, oblw::Run {v: 0, ct: 561}]) ,
		}
	}
	// push pause
	y.push(oblw::Run {v: 0, ct: 5000});
	let syn = oblw::v2b(oblw::rld(y));
	return syn
}

fn main() {
	let args = os::args();
	let mut timer = io::Timer::new().unwrap();
	let (mut c, p) = oblw::spawnBytestream(false);
	if ((args.len() - 1) > 0) {
		let n: uint = num::strconv::from_str_common(args[1].slice_from(0), 10, false, false, false, num::strconv::ExpNone, false, false).unwrap();
		for _ in range(0, 7) {c = oblw::sendBitstream(outlet(n), c); }
	}
	timer.sleep(100);
	unsafe {libc::funcs::c95::stdlib::exit(1);}
}
