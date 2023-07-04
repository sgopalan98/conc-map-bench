use std::hash::Hash;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use bustle::*;

use crate::bench::KeyValueType;

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Read { key: KeyValueType },
    Insert { key: KeyValueType, value: KeyValueType },
    Remove { key: KeyValueType },
    Increment { key: KeyValueType },
}

#[derive(Debug, Serialize, Deserialize)]
enum OperationResult {
    ReadSuccess(ResultData),
    ReadFailure(String),
    WriteSuccess(ResultData),
    WriteFailure(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum ResultData {
    String(String),
    Int(i32),
    Float(f64),
    // Add more types as needed
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerSettings {
    address: String,
    client_threads: usize,
    server_threads: usize,
    ops_per_req: usize,
    key_type: KeyValueType,
    value_type: KeyValueType,
    capacity: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerTable{
    server_settings: ServerSettings,
    stream: Option<TcpStream>
}

impl ServerTable {
    fn setup_server(address: String, client_threads: usize, server_threads: usize, ops_per_req: usize, key_type: KeyValueType, value_type: KeyValueType) -> Self {
        let server_settings = ServerSettings {
            address,
            client_threads,
            server_threads,
            ops_per_req,
            key_type,
            value_type,
            capacity: 0,
        };
        let mut stream = TcpStream::connect(address).expect("Failed to connect to server");
        return Self(server_settings, Some(stream));
    }
}

fn send_request<T: Serialize>(stream: &mut TcpStream, request: &T) {
    let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
    stream.write_all(request_json.as_bytes()).expect("Failed to send request");
}

impl Collection for ServerTable
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;

    fn with_capacity(_capacity: usize) -> Self {
        // this function not used
        Self(ServerSettings { address: "".to_owned(), client_threads: 0, server_threads: 0, ops_per_req: 0, key_type: 0, value_type: 0, capacity: 0 }, None)
    }

    fn reserve(&mut self, additional_capacity: usize) {
        self.server_settings.capacity = additional_capacity;
        send_request(&mut self.stream.unwrap(), &self.server_settings);
    }

    fn pin(&self) -> Self::Handle {
        let address = self.server_settings.address;
        let stream = TcpStream::connect(address).unwrap();
        Self(self.server_settings.clone(), Some(stream))
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

impl CollectionHandle for ServerTable
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        return true;
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        return true;
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        return true;
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        return true;
    }

    fn execute_multiple(&mut self, operation_types: Vec<OperationType>, keys: Vec<&Self::Key>) -> Vec<bool> {
        let mut operations = vec![];
        for index in 0..operation_types.len() {
            let operation_type = operation_types[index];
            let key = keys[index];
            let operation = match operation_type {
                OperationType::Read => Operation::Read { key },
                OperationType::Insert => Operation::Insert { key, value: 0 },
                OperationType::Remove => Operation::Remove { key },
                OperationType::Update => Operation::Increment { key},
                OperationType::Upsert => Operation::Read { key }, // TODO: Change this
            };
            operations.push(operation);
        }
        let mut stream = self.stream.unwrap();
        send_request(&mut stream, &operations);
    }
}
