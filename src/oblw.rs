extern crate usb;
extern crate libusb;
extern crate collections;

use std::comm;
use collections::bitv;

use std::comm::{Data, Empty, Disconnected};

#[deriving(Clone, DeepClone)]
pub struct Run {
	v: uint,
	ct: uint
}

pub fn rle<T: Ord+Clone+Primitive+ToPrimitive>(In: ~[T]) -> ~[Run] {
	let mut Out: ~[Run] = ~[Run { v: In[0].clone().to_uint().unwrap(), ct: 0u }];
	for i in In.iter() {
		if (i.to_uint().unwrap() == Out.last().unwrap().v) {
			Out[Out.len()-1].ct += 1;
		}
		else {
			Out.push(Run {v: i.clone().to_uint().unwrap(), ct: 1u });
		}
	};
	return Out
}

pub fn rld(In: ~[Run]) -> ~[uint] {
	let mut Out: ~[uint] = ~[];
	for i in In.iter() {
		for a in range(0u, i.ct.clone()) {
			Out.push(i.v.clone());
		}
	};
	return Out
}

pub fn B2b(bytes: ~[u8]) -> bitv::Bitv {
	return bitv::from_bytes(bytes)
}

pub fn v2b(uints: &[uint]) -> bitv::Bitv {
	let y: ~[bool] = uints.iter().map(|&x| x == 1u).to_owned_vec();
	return bitv::from_bools(y.slice_from(0))
}

pub fn b2B(bits: bitv::Bitv) -> ~[u8] {
	return bits.to_bytes()
}

pub fn r2b(runs: ~[Run]) -> bitv::Bitv {
	return v2b(rld(runs))
}

fn assemblePacket(y: &mut [u8], x: &[u8], norm: bool) {
	match norm {
		true => for i in range(0, x.len()) {y[i] = x[i];},
		false => for i in range(0, x.len()) {y[i] = x[i]^255u8;}
	}
}

pub fn spawnBytestream(defaultState: bool) -> (std::comm::Chan<~[u8]>, std::comm::Port<~[u8]>)  {
	let c = usb::Context::new();
	c.setDebug(2);
	let dev = match c.find_by_vid_pid(0x59e3, 0xf000) {
		Some(x) => x,
		None => fail!("no dev found"),
	};
	let handle = match dev.open() {
		Ok(x) => x,
		Err(code) => fail!("cannot open device {}", code),
	};
	handle.claim_interface(0);
	let ho = handle.clone();
	match defaultState {
		// low state - gnd - turn on inversion
		false => handle.ctrl_read(0x40|0x80, 0x08, 0x40, 0x0653, 0).unwrap(), // 0x693 - PINCTRL3; 0x40 - PORT_INVEN_bm
		// high value - 3v3 - turn off inversion
		true => handle.ctrl_read(0x40|0x80, 0x08, 0x00, 0x0653, 0).unwrap(),
	};

	let (pDataO, cDataO): (comm::Port<~[u8]>, comm::Chan<~[u8]>) = comm::Chan::new();
	spawn(proc() {
		// 0x02 = 
		ho.write_stream(0x02, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64, 8, |buf| {
			let y = buf.unwrap();
			match pDataO.try_recv() {
				Data(d) => {assemblePacket(y, d, defaultState); return true;},
				Empty => {
					match defaultState {
						true => assemblePacket(y, [0xffu8, ..64], defaultState),
						false => assemblePacket(y, [0x00u8, ..64], defaultState)
					};
					return true;
				},
				Disconnected => {
					return false;
				},
			}
		}); });

	let (pDataI, cDataI): (comm::Port<~[u8]>, comm::Chan<~[u8]>) = comm::Chan::new();
	let hi = handle.clone();
	spawn(proc() {
		hi.read_stream(0x81, libusb::LIBUSB_TRANSFER_TYPE_BULK, 64, 8, |res| {
			let y: ~[u8] = res.unwrap().to_owned();
			if cDataI.try_send(y) { true }
			else { false }
		})
	});
	return (cDataO, pDataI)
}

pub fn sendBitstream(syn: bitv::Bitv, c: comm::Chan<~[u8]>) -> comm::Chan<~[u8]>{
	let bytes: ~[u8] = b2B(syn);
	for packet in bytes.chunks(64) {
		let mut packet: ~[u8] = packet.iter().map(|&x| x).collect();
		let len = packet.len();
		packet.grow(64-len, &0x00);
		c.send(packet);
	}
	return c
}
