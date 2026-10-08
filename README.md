# auto-scaling-queued-workers-rs

A small Rust program that starts a worker for each id you send it.

Each worker has its own queue and handles one message every 4 seconds.
If a worker gets no messages for 10 seconds, it shuts itself down.
The next message for that id starts a new worker.

## Requirements

Rust and Cargo. Install them from https://rustup.rs.

## Run

```
cargo run
```

Type a message in this format and press Enter:

```
id,message
```

To send more than one message at once, separate them with `;`:

```
1,hello;2,world
```

Type `exit` to quit. If a worker is still running, the program closes once that worker times out.

## Test

There are no automated tests. Use these steps to check the program by hand.

1. Start the program with `cargo run`.
2. Type `1,first;1,second;2,other` and press Enter.
3. Workers 1 and 2 start right away.
4. After 4 seconds, worker 1 prints `first` and worker 2 prints `other`.
5. After 8 seconds, worker 1 prints `second`.
6. Wait 10 more seconds. Both workers print that they are disposed.
7. Type `1,again`. A new worker 1 starts and prints `again` after 4 seconds.
8. Type `exit` to quit.
