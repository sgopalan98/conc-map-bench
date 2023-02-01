use std::hash::Hash;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::task::ready;

use bustle::*;

pub struct ServerTable<K>(Option<TcpStream>, K);

impl<K> Collection for ServerTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
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

pub fn read_command(stream: &mut TcpStream) -> String {
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
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let mut command = vec![0u8; 9];
        command[0] = 1;
        command.splice(1..9, key.to_be_bytes());
        stream.write(&command);
        let mut buf = vec![0u8; 1];
        let result = stream.read_exact(&mut buf);
        let error_code = buf[0];
        error_code == 0
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let mut command = vec![0u8; 9];
        command[0] = 2;
        command.splice(1..9, key.to_be_bytes());
        stream.write(&command);
        let mut buf = vec![0u8; 1];
        let result = stream.read_exact(&mut buf);
        let error_code = buf[0];
        error_code == 0
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let mut command = vec![0u8; 9];
        command[0] = 3;
        command.splice(1..9, key.to_be_bytes());
        stream.write(&command);
        let mut buf = vec![0u8; 1];
        let result = stream.read_exact(&mut buf);
        let error_code = buf[0];
        error_code == 0
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let mut command = vec![0u8; 9];
        command[0] = 4;
        command.splice(1..9, key.to_be_bytes());
        stream.write(&command);
        let mut buf = vec![0u8; 1];
        let result = stream.read_exact(&mut buf);
        let error_code = buf[0];
        error_code == 0
    }

    fn execute(&mut self, operations: Vec<u8>, keys: Vec<&u64>) -> Vec<bool> {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let mut command = vec![0u8; 9 * 100];
        for index in 0..operations.len() {
            let start_index = 9 * index;
            let end_index = 9 * index + 9;
            command[9 * index] = operations[index];
            command.splice((start_index + 1)..end_index, keys[index].to_be_bytes());
        }
        stream.write(&command);
        let mut buf = vec![0u8; 100];
        let result = stream.read_exact(&mut buf);
        let mut results = vec![];
        for error_code in buf {
            results.push(error_code == 0);
        }
        return results;
    }

    fn close(&mut self) {
        let mut stream = self.0.as_mut().expect("TCPSTREAM SHOULD BE FOUND");
        let command = vec![0u8; 9];
        stream.write(&command);
        let mut buf = vec![0u8; 1];
        let result = stream.read_exact(&mut buf);
    }
}
