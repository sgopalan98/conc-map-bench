use std::hash::{BuildHasher, Hash};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::net::TcpStream;
use std::sync::Arc;

use bustle::*;

pub struct DashMapTable<K>(TcpStream, K);

impl<K> Collection for DashMapTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        let mut stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        let command = format!("RESET {} 0\n", 0);
        write_string(&mut stream, command);
        let result = read_command(&mut stream);
        assert!(result.eq("0"));
        Self(stream, 0.into())
    }

    fn pin(&self) -> Self::Handle {
        let stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        Self(stream, 0.into())
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

impl<K> CollectionHandle for DashMapTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Key = u128;

    fn get(&mut self, key: &Self::Key) -> bool {
        let command = format!("GET {} 0\n", key);
        write_string(&mut self.0, command);
        let result = read_command(&mut self.0);
        result.eq("0")
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let command = format!("INSERT {} {}\n", key, 0);
        write_string(&mut self.0, command);
        let result = read_command(&mut self.0);
        result.eq("0")
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let command = format!("REMOVE {} 0\n", key);
        write_string(&mut self.0, command);
        let result = read_command(&mut self.0);
        result.eq("0")
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let command = format!("UPDATE {} 0\n", key);
        write_string(&mut self.0, command);
        let result = read_command(&mut self.0);
        result.eq("0")
    }

    fn finish(&mut self) {
        println!("CALLING FINISH");
        let command = format!("FINISH {} 0\n", 0);
        write_string(&mut self.0, command);
    }
}
