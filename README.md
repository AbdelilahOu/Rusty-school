# RustThingy

## WHATS THIS

This is school management system build using:

- actix-web as backend server
- postgres as database
- sea-orm and sea-migration to connect and run migrations

This is how the schema looks like:

![database schema](school-management-db.svg)

You can also find the schema in the schema.sql file

Im using docker to run my database if you wanna use an other database you can here are the env variables that you need to run the project :

- DATABASE_URL
- OAUTH_CLIENT_ID
- OAUTH_SECRET
- REDIRECT_URL
- RANDOM_KEY
