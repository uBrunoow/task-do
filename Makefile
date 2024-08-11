runserver:
	cargo run

stopdb: 
	sudo docker compose -f docker-compose.yml down

rundb:
	sudo docker compose -f docker-compose.yml up -d --build