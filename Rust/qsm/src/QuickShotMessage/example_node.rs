use std::env;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::select;
use super::qsm::*;
use super::example_msgs::*;

async fn handle_client(mut socket: TcpStream, mut receiver: Receiver<String>, mut sender: Sender<String>) {
    let (mut reader, mut writer) = socket.split();
    let mut buffer = [0; 1024];

    loop {
        select! {
            result = reader.read(&mut buffer) => {
                let n = result.expect("Failed to read from socket");
                if n == 0 {
                    return;
                }

                let msg = String::from_utf8_lossy(&buffer[..n]);
                if msg.trim() == "exit" {
                    println!("Client requested disconnection.");
                    return;
                }
                
                sender.send(msg.to_string()).await.expect("Failed to send message");
            }
            Some(msg) = receiver.recv() => {
                println!("Received: {}", msg);
                writer.write_all(msg.as_bytes()).await.expect("Failed to write to socket");
            }
        }
    }
}

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind address");
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await.expect("Failed to accept connection");
        let (tx, rx) = mpsc::channel::<String>(100);
        tokio::spawn(handle_client(socket, rx, tx));
    }
}

pub async fn run_client() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.expect("Failed to connect to server");
    let (mut reader, mut writer) = socket.into_split();

    // Reader task
    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            let n = reader.read(&mut buffer).await.expect("Failed to read from server");
            if n == 0 {
                return;
            }
            let msg = String::from_utf8_lossy(&buffer[..n]);
            println!("Received: {}", msg);
        }
    });

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        if input.trim() == "exit" {
            println!("Exiting...");
            return;
        }
        else if input.trim() == "qTest" {
            let person = Person::new(1, QString::new("John".to_string()), QInteger::new(14), QFloat::new(172.3),
            QArray::new(vec![10, 32, 47], QType::QInt));

            let mut person_message = person.message_build();
            let mut s_message = seirialize(person_message);
            writer.write_all(s_message.as_bytes()).await.expect("Failed to write to server");
        }
        else
        {
            writer.write_all(input.as_bytes()).await.expect("Failed to write to server");
        }


    }
}