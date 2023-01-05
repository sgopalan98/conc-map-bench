use std::hash::Hash;
use std::io::{BufReader, Write, BufRead};
use std::net::TcpStream;

use bustle::*;

pub struct ServerTable<K>(Option<TcpStream>, K);

impl<K> Collection for ServerTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Handle = Self;

    fn with_capacity(_capacity: usize) -> Self {
        Self(None, 0.into())
    }

    fn with_capacity_and_threads(capacity: usize, no_of_threads: usize) -> Self {
        let mut stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        let command = format!("{} {}\n", capacity, no_of_threads);
        write_string(&mut stream, command);
        drop(stream);
        Self(None, 0.into())
    }

    fn pin(&self) -> Self::Handle {
        let stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        Self(Some(stream), 0.into())
    }
}

pub fn read_command(stream: &mut TcpStream) -> String{
    let mut input = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut input).unwrap();
    let input: String = input.trim().to_owned();
    return input;
}

pub fn write_string(stream: &mut TcpStream, output: String) {
    stream.write(output.as_bytes()).unwrap();
}

impl<K> CollectionHandle for ServerTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Key = u128;

    fn get(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = format!("GET {} 0\n", key);
        write_string(&mut stream, command);
        let result = read_command(&mut stream);
        result.eq("0")
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = format!("INSERT {} {}\n", key, 0);
        write_string(&mut stream, command);
        let result = read_command(&mut stream);
        result.eq("0")
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = format!("REMOVE {} 0\n", key);
        write_string(&mut stream, command);
        let result = read_command(&mut stream);
        result.eq("0")
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = format!("UPDATE {} 0\n", key);
        write_string(&mut stream, command);
        let result = read_command(&mut stream);
        result.eq("0")
    }

    fn close(&mut self) {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = format!("CLOSE {} 0\n", 0);
        write_string(&mut stream, command);
    }
}
