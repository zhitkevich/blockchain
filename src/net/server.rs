use crate::net::message::Message;
use crate::net::Node;
use log::{error, info, trace, warn};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};
use std::{io, thread};

pub fn start(port: u16) -> io::Result<()> {
	let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
	for stream in listener.incoming() {
		match stream {
			Ok(s) => {
				thread::spawn(move || handle(s));
			}
			Err(e) => error!("Failed to accept connection: {e}"),
		}
	}
	Ok(())
}

fn handle(mut stream: TcpStream) {
	let mut buffer = [0; 512];
	match stream.read(&mut buffer) {
		Ok(size) => match Message::from_le_bytes(&buffer[..size]) {
			Some(Message::Ping) => handle_ping(&mut stream),
			Some(Message::Block(block)) => info!("Received block: {block}"),
			_ => (),
		},
		Err(e) => error!("Failed to read stream: {e}"),
	}
}

fn handle_ping(stream: &mut TcpStream) {
	if let Err(e) = stream.write_all(&Message::Pong.to_le_bytes()) {
		match stream.peer_addr() {
			Ok(addr) => error!("Failed to send pong to {addr}: {e}"),
			Err(e) => error!("Failed to get peer address: {e}"),
		}
	}
}

pub fn ping<N: AsRef<[Node]>>(nodes: N, interval: Duration) {
	let nodes = nodes.as_ref();
	let mut last_ping = Instant::now();

	loop {
		thread::sleep(interval.saturating_sub(last_ping.elapsed()));
		last_ping = Instant::now();

		for node in nodes {
			let mut stream = match TcpStream::connect(&node.addr) {
				Ok(s) => s,
				Err(e) => {
					warn!("Failed to connect to {}: {e}", node.addr);
					continue;
				}
			};

			if let Err(e) = stream.write_all(&Message::Ping.to_le_bytes()) {
				error!("Failed to send ping to {}: {e}", node.addr);
				continue;
			}

			let mut buffer = [0; 512];
			match stream.read(&mut buffer) {
				Ok(size) => {
					if let Some(Message::Pong) = Message::from_le_bytes(&buffer[..size]) {
						trace!("Received pong from {}", node.addr);
					}
				}
				Err(e) => error!("Failed to read stream: {e}"),
			}
		}
	}
}
