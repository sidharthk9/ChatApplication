use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
	const LOCALHOST: &str = "127.0.0.1:6000";
	const MESSAGE_SIZE: usize = 32;
}
