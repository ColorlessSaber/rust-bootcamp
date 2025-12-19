<a id="readme-top"></a>

<!-- Project Title -->
<br>
<div>
    <h1 style="text-align:center">Microservice Project</h1>
</div>

<!-- Table of Contents -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#project-details">Project Details</a></li>
        <li><a href="#setup-running-in-ide">Setup - Running in IDE</a></li>
        <li><a href="#setup-running-in-docker">Setup - Running in Docker</a></li>
    </ol>
</details>
<br>

<!-- Project Details -->
## Project Details
A simple microservice project that implements an authentication, health-status, and a client CLI.<br>
The main goal of this project is to understand how to use Rust to create a microservice and have it run in a Docker
container.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Setup -->
## Setup - Running in IDE
1. Have a Rust IDE install along with Rust compiler on your computer.
2. Download the project from this repo and save it somewhere on your computer.
3. Open the project in your IDE and run the following commands in order.

First step is to build the Rust project.
```bash
cargo build
```

Next is to install watchexec to allow us to minor the activity of the authentication and health-status binary crates.
```bash
cargo install --locked watchexec-cli
```

With the project built and watchexec installed we can now start to run the authentication and health-status in separate
terminals. The first one to start up is the authentication.
```bash
 watchexec -c -q -w src/auth-service -e rs "cargo run -q --bin auth"
```

The next one to start up is the health-status. Again, run this code in a separate terminal.
```bash
watchexec -c -q -w src/health-check-service -e rs "cargo run -q --bin health-check"
```

You should start seeing messages in the authentication terminal and health-status terminal, an indication that things are
working!<br>
Lastly, we can run client CLI commands in a third terminal. To see the commands available, run the following command.
```bash
./target/debug/client help
```
Try signing up, signing in, and signing out. Below is an example of signing up.
To run a command, use the following as an example for sign-up, sign-in, and sign-out.
```bash
./target/debug/client sign-up --username bob --password 1234
```

## Setup - Running in Docker
1. Download and install Docker desktop app.
2. With Docker app installed and running on your computer open a terminal and cd to where you have saved the copy of
this repo on your computer.
3. Type in the following command to run the project in Docker.
```bash
docker-compose up
```
It will take a minute or two, but once it is up you will start seeing in the terminal messages being printed from
the authentication and health-status. When you are finished simple press ctrl+c to stop the containers in the terminal.

<p align="right">(<a href="#readme-top">back to top</a>)</p>