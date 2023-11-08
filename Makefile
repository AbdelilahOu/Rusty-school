dev: 
	cargo run
containerup:
	docker run --name school-manager-api -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

containerdown:
	docker stop school-manager-api
	docker rm --force school-manager-api

createdb: 
	docker exec -it school-manager-api createdb --username=root --owner=root school

dropdb:
	docker exec -it school-manager-api dropdb school

migrationup:
	sea-orm-cli migrate up

entities:
	sea-orm-cli generate entity --database-url postgresql://root:mysecretpassword@localhost:5432/school?sslmode=disable 