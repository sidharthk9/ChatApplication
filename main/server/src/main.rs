use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

fn sleep() {
	let duration = ::std::time::Duration::from_millis(100);
	thread::sleep(duration);
}

fn main() {
	const LOCALHOST: &str = "127.0.0.1:6000";
	const MESSAGE_SIZE: usize = 32;

	let server = TcpListener::bind(LOCALHOST).expect("Error: Failed to bind");
	server.set_nonblocking(true).expect("Error: Nonblocking Failed");

	let mut clients = vec![];
	let (transmitter, receiver) = mpsc::channel::<String>();
	loop {
		if let Ok((mut socket, address)) = server.accept() {
			println!("Client from {0} connected", address);

			let single_transmitter = transmitter.clone();
			clients.push(socket.try_clone().expect("Error: failed to include new client"));

			thread::spawn(move || loop {
				let mut buffer = vec![0; MESSAGE_SIZE];
				match socket.read_exact(&mut buffer) {
					Ok(_) => {
						let iterable_content = buffer.into_iter().take_while(|x| x != 0)
							.collect::<Vec<_>>();
						let message = String::from_utf8(iterable_content)
							.expect("Error: invalid Message");

						println!("{0}: {1}", address, message);
						transmitter.send(message).expect("Error: Message did not reach receiver");
					},

					Err(ref error) if error.kind() = ErrorKind::WouldBlock => (),

					Err(_) => {
						println!("Closing connection with {0}", address);
						break;
					},
				}

				sleep();
			});
		}

		if let Ok(message) = receiver.try_recv() {
			clients = clients.into_iter()
				.filter_map( |mut client| {
					let mut buffer = message.clone().into_bytes();
					buffer.resize(MESSAGE_SIZE, 0);

					client.write_all(&buffer).map(|_| client).ok()
				}).collect::<Vec<_>>();
		}

		sleep();
	}
}
