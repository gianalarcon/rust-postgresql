use postgres::{Client, Error, NoTls};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
pub mod model;
pub mod statements;
#[macro_use]
extern crate serde_derive;

fn main() -> Result<(), Error> {
    set_database()?;
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(e) => eprintln!("Error handling client side: {}", e),
        }
    }
    Ok(())
}

fn connect_to_database() -> Result<Client, postgres::Error> {
    let client = Client::connect(
        "postgresql://gianm:system14@localhost:5432/rust_api_database",
        NoTls,
    )?;
    Ok(client)
}
fn set_database() -> Result<(), Error> {
    let mut client = connect_to_database()?;
    let query = statements::CREATE_DB;
    client.batch_execute(query)?;
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);

            let (status_line, content) = if request.starts_with("GET /users") {
                handle_get_all_request()?
            } else if request.starts_with("POST /users") {
                handle_post_request(&request)?
            } else if request.starts_with("DELETE /users") {
                handle_delete_request(&request)?
            } else if request.starts_with("PUT /users") {
                handle_put_request(&request)?
            } else {
                (
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_owned(),
                    "404 Not Found".to_owned(),
                )
            };
            let response = format!("{}{}", status_line, content);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            print!("Error: {}", e);
        }
    }
    Ok(())
}
fn handle_put_request(request: &str) -> Result<(String, String), Error> {
    update_one(request)?;
    Ok((
        "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
        format!("Updated user"),
    ))
}

fn update_one(request: &str) -> Result<(), Error> {
    let request_body = request.split("\r\n\r\n").last().unwrap_or("");
    let user: model::User = serde_json::from_str(request_body).unwrap();

    let mut request_split = request.split(" ");
    let mut request_split: Vec<&str> = request_split
        .nth(1)
        .unwrap()
        .split(|e| e == '=' || e == '?')
        .collect();
    let id = request_split.pop().unwrap();
    let id = id.parse::<i32>().unwrap();

    let mut client = connect_to_database()?;
    let query = statements::UPDATE_USER;
    client.execute(query, &[&id, &user.username, &user.password, &user.email])?;
    Ok(())
}

fn handle_delete_request(request: &str) -> Result<(String, String), Error> {
    delete_one(request)?;
    Ok((
        "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
        format!("Deleted user"),
    ))
}

fn delete_one(request: &str) -> Result<(), Error> {
    let mut request_split = request.split(" ");
    let mut request_split: Vec<&str> = request_split
        .nth(1)
        .unwrap()
        .split(|e| e == '=' || e == '?')
        .collect();
    let id = request_split.pop().unwrap();
    let id = id.parse::<i32>().unwrap();

    let mut client = connect_to_database()?;
    let query = statements::DELETE_USER;
    client.execute(query, &[&id])?;
    Ok(())
}
fn handle_get_all_request() -> Result<(String, String), Error> {
    let mut users: Vec<model::User> = Vec::new();

    let mut client = connect_to_database()?;
    let query = statements::SELECT_ALL_USERS;
    for row in client.query(query, &[]).unwrap() {
        let username = row.get(0);
        let password = row.get(1);
        let email = row.get(2);

        let user = model::User {
            username,
            password,
            email,
        };

        users.push(user);
    }

    let user_json = serde_json::to_string(&users).unwrap();
    Ok(("HTTP/1.1 200 OK\r\n\r\n".to_owned(), user_json))
}
fn handle_post_request(request: &str) -> Result<(String, String), Error> {
    let request_body = request.split("\r\n\r\n").last().unwrap_or("");

    create_one(request_body)?;
    Ok((
        "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
        format!("Received data: {}", request_body),
    ))
}

fn create_one(request_body: &str) -> Result<(), Error> {
    let user: model::User = serde_json::from_str(request_body).unwrap();
    let mut client = connect_to_database()?;

    let query = statements::INSERT_USER;
    client.execute(query, &[&user.username, &user.password, &user.email])?;
    Ok(())
}
