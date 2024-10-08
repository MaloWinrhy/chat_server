
# chat_server

## Description

`chat_server` is a simple chat server written in Rust, designed to run in the command line. This project was created as a way to learn and understand the basics of asynchronous network programming in Rust. The server allows multiple clients to connect simultaneously, choose a nickname, and communicate in real-time via text messages.

## Features

- **Nickname Management**: Each user must choose a nickname using the `/nick <your_nickname>` command before they can send messages.
- **Message Broadcasting**: All messages sent by a user are broadcasted to all other connected users.
- **Built-in Commands**:
  - `/nick <your_nickname>`: Allows users to choose or change their nickname.

## Requirements

To run this server, you need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

## Installation and Usage

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/chat_server.git
   cd chat_server
   ```

2. **Build and run the server**:
   ```bash
   cargo run
   ```

3. **Connect to the server**:
   Use a client like Telnet or Netcat to connect to the server:
   ```bash
   telnet 127.0.0.1 8080
   ```
   or
   ```bash
   nc 127.0.0.1 8080
   ```

4. **Choose a nickname**:
   Upon connection, choose a nickname using the command:
   ```bash
   /nick <your_nickname>
   ```

5. **Send messages**:
   After choosing a nickname, type your messages and press Enter to send them. All connected users will see your message.

## Project Archived

This project is now archived and will no longer receive updates or new features. It remains available in a read-only state for reference.

## Author

Created by [Malo HENRY](https://github.com/MaloWinrhy).
