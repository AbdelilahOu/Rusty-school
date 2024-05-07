db-container:
	docker run --name school-db --network school-backend -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

api-container:
	docker run --name school-manager-api --network school-backend

createdb: 
	docker exec -it school-db createdb --username=root --owner=root school

dropdb:
	docker exec -it school-db dropdb school

migrationsup:
	sea-orm-cli migrate up -u postgres://root:mysecretpassword@localhost:5432/school

migrationslatest:
	sea-orm-cli migrate down -u postgres://root:mysecretpassword@localhost:5432/school

migrationsdown: 
	sea-orm-cli migrate reset -u postgres://root:mysecretpassword@localhost:5432/school

migrationsfresh:
	sea-orm-cli migrate fresh -u postgres://root:mysecretpassword@localhost:5432/school

entity:
	del /S C:\Users\abdel\OneDrive\Desktop\Projects\Personal\rust\school-management-api\entity\src\*.rs
	sea-orm-cli generate entity -o entity/src --lib -u postgres://root:mysecretpassword@localhost:5432/school

server:
	cargo run

build:
	cargo build --verbose --workspace --release

.PHONY: watch build server createdb dropdb mup mdownlatest mdownall mdownfresh mdownrefresh entity
