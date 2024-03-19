dbcontainerup:
	docker run --name school-db --network school-backend -e POSTGRES_USER=root -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres:15

apicontainerup:
	docker run --name school-manager-api --network school-backend -e DATABASE_URL="postgresql://root:mysecretpassword@school-db:5432/school?sslmode=disable" -e OAUTH_CLIENT_ID="688282033704-2leacl36je751emdajmd7n2c8u3gnqrs.apps.googleusercontent.com" -e OAUTH_SECRET="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" -e REDIRECT_URL="http://127.0.0.1:8080/auth/sessions/google" -e RANDOM_KEY="be11e551-6c13-419a-a1f2-cb8f6cf476d4" -e JWT_MAX_AGE=48 -e RUST_LOG='simple-auth-server=debug,actix_web=info,actix_server=info'  -p 8080:8080 -d school-manager-api:latest

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
	sea-orm-cli generate entity -o entity/src --lib -u postgres://root:mysecretpassword@localhost:5432/school

server:
	cargo run

watch:
	cargo watch -x run

build:
	cargo build --verbose --workspace --release

.PHONY: watch build server createdb dropdb mup mdownlatest mdownall mdownfresh mdownrefresh entity
