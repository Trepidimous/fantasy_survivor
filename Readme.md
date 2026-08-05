<style>
.green1 {
    color:rgba(104, 255, 96, 0.78);
}
.red1 {
    color:rgba(255, 122, 96, 0.96);
}
.purple1 {
    color:rgba(228, 74, 255, 1);
}
.blue1 {
    color:rgba(62, 178, 224, 1);
}
.orange1 {
    color: rgba(228, 167, 13, 0.95);
}
</style>

<h2>Overview</h2>

This readme is for getting started with this repo.  

## Table of Contents
[Fantasy Survivor Design](#design)  
[Instructions](#instructions)  
[Tech Stack](#tech-stack)  
[Setup](#setup)  
[How to Run](#how-to-run)  
[Troubleshooting](#troubleshooting)  
[Postman](#postman)  

<h3 id="design" style="color:rgb(242, 242, 242);">Fantasy Survivor Design</h3>

https://docs.google.com/document/d/1VQA3wyLsVLK_2MhDMflwM6awSolw7cBsfKcDlPFCg9I/edit?tab=t.0

<h3 id="instructions" style="color:rgb(242, 242, 242);">Instructions</h3>

https://www.youtube.com/watch?v=FYVbt6YFMsM

His github:  
https://github.com/FrancescoXX/rust-fullstack-app

<h3 id="tech-stack" style="color:rgb(242, 242, 242);">Tech Stack</h3>

<span class="green1">Front End</span> - Rust as programming language.  
WebAssembly to run Rust in the browser.  
Yew as frontend framework in Rust.  
Trunk as build tool for Yew applications.  
Tailwind CSS as a utility-first CSS framework.  

<span class="green1">Backend</span> - Rocket as web framework in Rust.  
Rust as programming language for backend logic.  
This will be used for Rest API's.  

<span class="green1">Database</span> - PostgreSQL as our relational database engine.  
Docker - to run PostgreSQL in a container.  

<h3 id="setup" style="color:rgb(242, 242, 242);">Setup</h3>

1) Clone this github repo to your local machine.  

2) Check if you already have Rust and Cargo installed with:  
`rustc -V` and `cargo -V`

    To install Rust, go to: https://rustup.rs/ and Download. Then Install.  
    Restart terminal and VSCode. Then re-run above commands to ensure it's working.  
    To update Rust, type: `rustup update`  

3) Install Docker Desktop

4) Open a command prompt terminal.
Then navigate to your repo's directory on your local machine.  
For example, cd to: C:\Users\your_name\Desktop\Dev\fantasy_survivor\frontend

5) Check the local package and it's dependencies for errors by typing: `cargo check`  

6) In the same directory, run: `cargo new frontend --vcs none`  

    This tells cargo to create a new Rust project in a folder called 'frontend'.  
    It creates a Cargo.toml file, which is the project manifest.  
    It also creates a src/main.rs file.  

    --vcs none tells Cargo to skip creating a new git folder in the dir.  

7) In the same directory, install Yew by running: `rustup target add wasm32-unknown-unknown`  

    Yew is the front end Rust framework. wasm32 means 32-bit web assembly architecture.  
    The 1st unknown means no specific OS.  
    The 2nd unknown means no specific system libraries.  

    Unlike pip package manage in Python, Cargo does NOT store global packages.  
    Every crate is installed per project and lives in that project's target folder.  
    The closest way to check cargo is to look at the Cargo.toml file or run: `cargo tree`  

8) In the same directory, install Trunk by running: `cargo install trunk --locked`  
    When it finishes, verify Trunk by typing: `trunk --version`  

<h3 id="how-to-run" style="color:rgb(242, 242, 242);">How to Run</h3>

1. Open the Docker Desktop program  

2. Start PostgreSQL in a docker container by running: `docker compose up`  
    Run this from a terminal in this repo's main directory.  

3. Connect to this database by:  
    a. psql connection.  
   To do this, open another terminal in this repo's main directory.  
   Create a new terminal session inside that container by running: `docker exec -it db psql -U postgres`  
   To exit psql, type: `\q`  
    b. Or manually connect with DBeaver and enter database connection credentials.    

4. Run the backend server (locally)  
    a. Open a command prompt from this repo's directory. cd to `/backend` directory.  
    b. run `cargo build`  
    c. run `cargo run`  

5. Build front-end by:  
    a. Open a command prompt from this repo's directory. cd to `/frontend` directory  
    b. run `cargo build --target wasm32-unknown-unknown`

6. Start front-end with:  
    a. run `trunk serve`

7. Go to URL: http://127.0.0.1:8080/gamemaster-portal
And http://127.0.0.1:8080/player-portal

The root URL for this application is: http://127.0.0.1:8080  
However, this shows and does nothing.  

8. Review records in the database by connecting to it with DBeaver or another database GUI.  

<h3 id="troubleshooting" style="color:rgb(242, 242, 242);">Troubleshooting</h3>

If the database needs to get recreated for any reason, run:  
  `docker compose down -v`  
  `docker compose up`  

<h3 id="postman" style="color:rgb(242, 242, 242);">Postman</h3>

To list the contestants in Postman, run:
  GET  
  http://127.0.0.1:8000/api/contestants  

To read the values of users in Postman, run:
  GET
  http://127.0.0.1:8000/api/users

This is NOT how you should get information from a database.  
You should use a GUI like DBeaver or PGAdmin.  
