color A

start "" "https://docs.google.com/document/d/1VQA3wyLsVLK_2MhDMflwM6awSolw7cBsfKcDlPFCg9I/edit?tab=t.0"

start "" "C:/Program Files/Docker/Docker/Docker Desktop.exe"

TIMEOUT /T 10 /NOBREAK

cd C:/source/software/fantasy_survivor

start cmd /k "color A & echo Starting Up Our Database & docker compose up"

TIMEOUT /T 6 /NOBREAK

start cmd /k "color B & echo Starting PostGres & docker exec -it db psql -U postgres"

cd C:/source/software/fantasy_survivor/backend/src

start cmd /k "color A & cargo build & cargo run" 

cd C:/source/software/fantasy_survivor/frontend

start cmd /k "color A & cargo build --target wasm32-unknown-unknown & trunk serve"

TIMEOUT /T 3 /NOBREAK

start "" http://127.0.0.1:8080/gamemaster-portal
start "" http://127.0.0.1:8080/player-portal