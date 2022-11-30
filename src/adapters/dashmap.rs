use std::hash::{BuildHasher, Hash};
use std::io::BufReader;
use std::net::TcpStream;
use std::io::BufRead;
// use std::net::TcpStream;
// use std::sync::Arc;

use bustle::*;

use hashmap_server_mod::hash_map_client::HashMapClient;
use hashmap_server_mod::{HashMapRequest, HashMapReply};
use tonic::transport::Channel;
use futures::executor::block_on;


pub mod hashmap_server_mod {
    tonic::include_proto!("hashmap");
}

pub struct DashMapTable<K>(HashMapClient<Channel>, K);

impl<K> Collection for DashMapTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        let mut stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        let result = read_command(&mut stream);
        let addr = format!("http://{}", result);
        println!("ADDR IS {}", addr);
        let mut client = block_on(HashMapClient::connect(result)).unwrap();
        Self(client, 0.into())
    }

    fn pin(&self) -> Self::Handle {
        let mut stream = TcpStream::connect("0.0.0.0:7879").unwrap();
        let result = read_command(&mut stream);
        let addr = format!("http://{}", result);
        println!("ADDR IS {}", addr);
        let mut client = block_on(HashMapClient::connect(result)).unwrap();
        Self(client, 0.into())
    }
}

pub fn read_command(stream: &mut TcpStream) -> String{
    let mut input = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut input).unwrap();
    let input: String = input.trim().to_owned();
    return input;
}


impl<K> CollectionHandle for DashMapTable<K>
where
K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        // let command = format!("GET {} 0\n", key);
        // write_string(&mut self.0, command);
        // let result = read_command(&mut self.0);
        // result.eq("0")
        let request = tonic::Request::new(HashMapRequest{
            key: *key as i64,
        });
        let response = block_on(self.0.get(request)).unwrap();
        response.into_inner().error_code

    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let request = tonic::Request::new(HashMapRequest{
            key: *key as i64,
        });
        let response = block_on(self.0.insert(request)).unwrap();
        response.into_inner().error_code
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let request = tonic::Request::new(HashMapRequest{
            key: *key as i64,
        });
        let response = block_on(self.0.remove(request)).unwrap();
        response.into_inner().error_code
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let request = tonic::Request::new(HashMapRequest{
            key: *key as i64,
        });
        let response = block_on(self.0.update(request)).unwrap();
        response.into_inner().error_code
    }

    fn finish(&mut self) {
        let request = tonic::Request::new(HashMapRequest{
            key: 0,
        });
        let response = block_on(self.0.reset(request)).unwrap();
    }
}
