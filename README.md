# Brief explanation
- I'm setting up an axum server to run locally
- In another executable I then use reqwest to interact with that server
- I get the following output

```
[client/src/main.rs:19:5] &client = Client {
    accepts: Accepts,
    proxies: [
        Proxy(
            System(
                {
                    "http": http://127.0.0.1:49765,
                    "https": http://127.0.0.1:49765,
                },
            ),
            None,
        ),
    ],
    referer: true,
    default_headers: {
        "accept": "*/*",
    },
}
2024-03-05T07:10:21.060078Z DEBUG reqwest::connect: starting new connection: http://0.0.0.0:8001/    
2024-03-05T07:10:21.060133Z DEBUG reqwest::connect: proxy(http://127.0.0.1:49765) intercepts 'http://0.0.0.0:8001/'    
2024-03-05T07:10:21.060211Z TRACE hyper::client::connect::http: Http::connect; scheme=Some("http"), host=Some("127.0.0.1"), port=Some(Port(49765))
2024-03-05T07:10:21.060280Z DEBUG hyper::client::connect::http: connecting to 127.0.0.1:49765
2024-03-05T07:10:21.060623Z DEBUG hyper::client::connect::http: connected to 127.0.0.1:49765
2024-03-05T07:10:21.060644Z TRACE hyper::client::conn: client handshake Http1
2024-03-05T07:10:21.060744Z TRACE hyper::client::client: handshake complete, spawning background dispatcher task
2024-03-05T07:10:21.061143Z TRACE hyper::proto::h1::conn: flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }
2024-03-05T07:10:21.061337Z TRACE encode_headers: hyper::proto::h1::role: Client::encode method=GET, body=None
2024-03-05T07:10:21.061480Z DEBUG hyper::proto::h1::io: flushed 100 bytes
2024-03-05T07:10:21.061521Z TRACE hyper::proto::h1::conn: flushed({role=client}): State { reading: Init, writing: KeepAlive, keep_alive: Busy }
2024-03-05T07:10:21.062385Z TRACE hyper::proto::h1::conn: Conn::read_head
2024-03-05T07:10:21.062401Z TRACE hyper::proto::h1::io: received 0 bytes
2024-03-05T07:10:21.062409Z TRACE hyper::proto::h1::io: parse eof
2024-03-05T07:10:21.062416Z TRACE hyper::proto::h1::conn: State::close_read()
2024-03-05T07:10:21.062423Z DEBUG hyper::proto::h1::conn: parse error (connection closed before message completed) with 0 bytes
2024-03-05T07:10:21.062431Z DEBUG hyper::proto::h1::dispatch: read_head error: connection closed before message completed
2024-03-05T07:10:21.062443Z TRACE hyper::proto::h1::conn: State::close_read()
2024-03-05T07:10:21.062450Z TRACE hyper::proto::h1::conn: State::close_write()
2024-03-05T07:10:21.062457Z TRACE hyper::proto::h1::conn: flushed({role=client}): State { reading: Closed, writing: Closed, keep_alive: Disabled }
2024-03-05T07:10:21.062475Z TRACE hyper::proto::h1::conn: shut down IO complete
thread 'main' panicked at client/src/main.rs:27:10:
called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(0.0.0.0)), port: Some(8001), path: "/", query: None, fragment: None }, source: hyper::Error(IncompleteMessage) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

# Run it yourself
- Clone the repo
- `cd` into it
- in one terminal `cargo r --bin server`
- and in another `cargo r --bin client`