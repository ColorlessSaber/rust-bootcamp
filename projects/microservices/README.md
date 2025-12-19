
## Running the system in terminal
Command to install watchexec, alternative to 'cargo watch.'
```bash
cargo install --locked watchexec-cli
```

Command to run the auth service
```bash
 watchexec -c -q -w src/auth-service -e rs "cargo run -q --bin auth"
```

Command to run the health-check
```bash
watchexec -c -q -w src/health-check-service -e rs "cargo run -q --bin health-check"
```

Command to see the available CLI commands for client
```bash
./target/debug/client help
```

To run a command, use the following as an example for sign-up, sign-in, and sign-out.
```bash
./target/debug/client sign-up --username bob --password 1234
```