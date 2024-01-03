


docker kill mongo-db-test
docker rm mongo-db-test


docker pull mongo:latest

docker run -d --name mongo-db-test --restart=always -p 27017:27017 -v mongodb_db_container:/data/db mongo:latest