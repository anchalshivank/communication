### Problem Overview

I am developing a communication layer that supports both **TCP** and **QUIC** protocols for data exchange between a client and a server. The implementation appears to work flawlessly in **TCP mode**, but in **QUIC mode**, there is an issue with data transmission despite a successful connection.

### Setup and Execution Commands

Use the following commands to test the setup in both modes:

1. **TCP Listener**: `cargo run -- listener tcp`
2. **TCP Client**: `cargo run -- conn tcp`
3. **QUIC Listener**: `cargo run -- listener quic`
4. **QUIC Client**: `cargo run -- conn quic`

### Observed Behavior

#### TCP Mode (Working Fine)

- **TCP Listener**: The server starts successfully, listens on the specified address, and properly receives data from the client.
- **TCP Client**: The client connects successfully to the server and exchanges messages, which are confirmed by the following output:

**TCP Listener Output:**
```
Running as tcp listener on 127.0.0.1:5000
Tcp listener started at 127.0.0.1:46220
Received from client: [72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 108, 105, 101, 110, 116]
```

**TCP Client Output:**
```
Running as tcp client connecting to 127.0.0.1:5000
Received from server: [72, 101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 115, 101, 114, 118, 101, 114]
```

#### QUIC Mode (Data Transmission Issue)

- **QUIC Listener**: The server starts and listens as expected. It successfully accepts a connection from the client, but it does not receive any meaningful data. Instead, it shows an empty array `[]` as the received message.
- **QUIC Client**: The client connects to the server without errors, but no data is actually exchanged between the client and server.

**QUIC Listener Output:**
```
QUIC server started on 127.0.0.1:5000
Running as quic listener on 127.0.0.1:5000
Connection received from 127.0.0.1:6000
Received from client: []
```

**QUIC Client Output:**
```
Attempting to connect to server at 127.0.0.1:5000
Connected to server
```

### Key Points of Concern

1. **Connection Success without Data Exchange**: The QUIC client and listener successfully establish a connection, as shown by the client’s “Connected to server” message and the listener’s “Connection received” message. However, no actual data is exchanged.
  
2. **Empty Data Received**: The QUIC listener indicates that it received an empty message (`[]`), suggesting that either the client did not send the data as expected or the server did not correctly read it.

3. **Working TCP Implementation**: The TCP implementation works as expected, which suggests that the general setup for message handling is correct, but there may be an issue specific to the QUIC setup or stream handling.


In summary, your TCP implementation works perfectly, but QUIC is encountering issues with data exchange despite a successful connection. Further investigation into QUIC’s stream handling, data flushing, and timing may help resolve the empty data issue.
