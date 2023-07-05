## Listen on unused port TCP/IP

[![std-badge]][std] [![cat-net-badge]][cat-net]

In this example, the port is displayed on the console, and the program will
listen until a request is made.  `SocketAddrV4` assigns a random port when
setting port to 0.

```rust,no_run
{{#include examples/listen_unused.rs}}
```

From the cookbook repository, run this example:

```
cargo run --example listen_unused
```

The listening port number will be printed, as in this example run:

```
Listening on 127.0.0.1:34337, access this port to end the program
```

Then, in another terminal window, use the `netcat` command to connect to
the example program, specifing the listening port number printed, and
send it some data:

```
echo "hi there" | nc -N localhost 34337
```

The example program should then print out what was sent over the TCP socket
and then exit:

```
Connection received! 127.0.0.1:36500 is sending data.
127.0.0.1:36500 says hi there
```
