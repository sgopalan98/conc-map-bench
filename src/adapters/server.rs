use std::hash::Hash;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use bustle::*;
use std::mem::size_of_val;

use crate::bench::KeyValueType;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandShakeRequest {
    client_threads: usize,
    server_threads: usize,
    ops_per_req: usize,
    capacity: usize,
    key_type: KeyValueType,
    value_type: KeyValueType
}

#[derive(Debug, Serialize, Deserialize)]
struct OperationRequests {
    operations: Vec<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OperationResults {
    results: Vec<OperationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Read { key: KeyValueType },
    Insert { key: KeyValueType, value: KeyValueType },
    Remove { key: KeyValueType },
    Increment { key: KeyValueType },
    Close
}

#[derive(Debug, Serialize, Deserialize)]
enum OperationResult {
    Success(KeyValueType),
    Failure
}

pub struct ServerTable{
    network_config: Option<NetworkConfig>,
    stream: Option<TcpStream>
}

fn send_request<T: Serialize>(stream: &mut TcpStream, request: &T) {
    let request_json = serde_json::to_string(&request).expect("Failed to serialize request");
    stream.write_all(request_json.as_bytes()).expect("Failed to send request");
}

fn receive_response(stream: &mut TcpStream) -> OperationResults {
    let mut buffer = [0; 1024 * 10];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response_json = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
    // println!("response json is {}", response_json);
    serde_json::from_str(&response_json).unwrap()
}

impl Collection for ServerTable
{
    type Handle = Self;

    fn with_capacity(_capacity: usize) -> Self {
        // not used
        Self{network_config: None, stream: None}
    }

    fn in_network_with_capacity(network_config: NetworkConfig, capacity: usize) -> Self {
        let address = network_config.address.clone();
        let mut stream = TcpStream::connect(address).unwrap();
        let handshake_request = HandShakeRequest {
            client_threads: network_config.client_threads,
            server_threads: network_config.server_threads,
            ops_per_req: network_config.ops_per_req,
            capacity,
            key_type: KeyValueType::Int(0),
            value_type: KeyValueType::Int(0)
        };
        send_request(&mut stream, &handshake_request);
        return Self{network_config: Some(network_config.clone()), stream: Some(stream) };
    }

    fn pin(&self) -> Self::Handle {
        let network_config = self.network_config.clone().unwrap();
        let address = network_config.address.clone();
        let stream = TcpStream::connect(address).unwrap();
        return Self{network_config: Some(network_config), stream: Some(stream) };
    }
}

impl CollectionHandle for ServerTable
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

        let mut return_data_available = true;
        // Send data for operations
        let mut operations = vec![];
        for index in 0..operation_types.len() {
            let operation_type = operation_types[index];
            let operation = match operation_type {
                OperationType::Read => {
                    let key = keys[index];
                    Operation::Read { key: KeyValueType::Int(*key) }
                },
                OperationType::Insert => {
                    let key = keys[index];
                    Operation::Insert { key: KeyValueType::Int(*key), value: KeyValueType::Int(0) }
                },
                OperationType::Remove => {
                    let key = keys[index];
                    Operation::Remove { key: KeyValueType::Int(*key) }
                },
                OperationType::Update => {
                    let key = keys[index];
                    Operation::Increment { key: KeyValueType::Int(*key)}
                },
                OperationType::Upsert => {
                    let key = keys[index];
                    Operation::Read { key: KeyValueType::Int(*key) }
                },
                OperationType::End => {
                    return_data_available = false; 
                    Operation::Close
                }
            };
            operations.push(operation);
        }
        let mut stream = self.stream.as_mut().unwrap();
        let operations_request = OperationRequests{
            operations
        };
        send_request(stream, &operations_request);

        if !return_data_available {
            return vec![];
        }

        // Evaulate the results
        let operation_results = receive_response(&mut stream);
        let mut result_booleans = vec![];
        for index in 0..operation_results.results.len() {
            let operation_result = &operation_results.results[index];
            let operation_type = operation_types[index];
            let result = match operation_type {
                OperationType::Read | OperationType::Remove | OperationType::Update => match operation_result {
                    OperationResult::Success(_) => true,
                    OperationResult::Failure => false,
                },
                OperationType::Insert => match operation_result {
                    OperationResult::Success(value) => match value {
                        KeyValueType::Int(value) => *value == 0,
                    },
                    OperationResult::Failure => false
                },
                OperationType::End => false,
                OperationType::Upsert => false,
            };
            result_booleans.push(result);
        }
        return result_booleans;
    }
}
