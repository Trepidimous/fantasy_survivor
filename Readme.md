{\rtf1}

[Designs]
https://docs.google.com/document/d/1VQA3wyLsVLK_2MhDMflwM6awSolw7cBsfKcDlPFCg9I/edit?tab=t.0

[Tech Stack Instructions]
https://www.youtube.com/watch?v=FYVbt6YFMsM

[Setup]
1. Install Docker Desktop

2. cd to "C:/source/software/fantasy_survivor/frontend"
  cargo new frontend --vcs none

3. Install yew (front end)
	rustup target add wasm32-unknown-unknown

4. Install trunk --locked

5. Setup FrontEnd
	cargo new frontend --vcs none

[How to Run]

1. Start docker desktop
2. Start a database
	a. Open a cmd from the "C:/source/software/fantasy_survivor" directory
	b. "docker compose up"
3. Start PostGres?
	a. Open a cmd from the "C:/source/software/fantasy_survivor" directory
	b. "docker exec -it db psql -U postgres"
4. Run our backend server (locally)
	a. Open a cmd from the "C:/source/software/fantasy_survivor/backend/src" directory
	b. "cargo build"
	c. "cargo run"

with postman, we can very that we can create, read, update, and destroy content in our database.
	* Read values:
	Get: http://127.0.0.1:8000/api/users

Database Viewers:
  DBeaver or PG Admin

5. Build FE
	C:/source/software/fantasy_survivor/frontend
	* cargo build --target wasm32-unknown-unknown

6. Start FE?
	* trunk serve

7. Go to http://127.0.0.1:8080/



https://github.com/FrancescoXX/rust-fullstack-app


# Troubleshooting

If the database needs to be re-created or gets corrupted
  docker compose down -v
  docker compose up