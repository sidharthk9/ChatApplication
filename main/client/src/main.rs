use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
	const LOCALHOST: &str = "127.0.0.1:6000";
	const MESSAGE_SIZE: usize = 32;

	let mut client = TcpStream::connect(LOCALHOST).expect("Error: Server Unconnected");
	client.set_nonblocking(true).expect("Error: Nonblocking Failed");

	let (transmitter, receiver) = mpsc::channel::<String>();

	thread::spawn(move || loop {
		let mut buffer = vec![0; MESSAGE_SIZE];

		match client.read_exact(&mut buffer) {
			Ok(_) => {
				let message = buffer.into_iter().take_while(|&x| x != 0)
					.collect::<Vec<_>>();

				println!("Received: {0:?}", message);
			},

			Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),

			Err(_) => {
				println!("Error: Server Connection is Severed");
				break;
			},
		}
		match receiver.try_recv() {
			Ok(message) => {
				let mut buffer = message.clone().into_bytes();
				buffer.resize(MESSAGE_SIZE, 0);
				client.write_all(&buffer).expect("Error: Message Transmission Failed");
				println!("Message Sent");
			},
			Err(TryRecvError::Empty) => (),
			Err(TryRecvError) => break,

		}

		thread::sleep(Duration::from_millis(100));
	});

	println!("Write a message: ");

	loop {
		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer).expect("Error: Failed Reading the Input");

		let message = buffer.trim().to_string();
		if message ==":quit" || transmitter.send(message).is_err() {
			break;
		}
	}
	println!("Goodbye!");
}
