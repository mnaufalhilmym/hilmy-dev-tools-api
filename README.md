1. Run Ubuntu
   sudo docker run --rm -v ubuntu-22.04-usr:/usr -v ubuntu-22.04-var:/var -v ubuntu-22.04-root:/root -v ubuntu-22.04-etc:/etc -v /media/hilmy/2de4c9c1-c1b4-430c-8e93-2bbd5c01f88e/labs/splashmodic/tools/tools-backend/services:/app --network=host -it ubuntu:22.04 bash
1. Run Postgres
   sudo docker run --rm -e POSTGRES_DB=tools -e POSTGRES_USER=tools -e POSTGRES_PASSWORD=JrLMp42N2yYE84 -v /run/media/hilmy/2de4c9c1-c1b4-430c-8e93-2bbd5c01f88e/labs/splashmodic/tools/tools-backend/docker/volumes/pg:/var/lib/postgresql/data -p 5432:5432 postgres:15-alpine
1. Run Redis
   sudo docker run --rm -p 6379:6379 redis:7-alpine
1. Run Apache Zookeeper
   sudo docker run --rm -e ALLOW_ANONYMOUS_LOGIN=yes -p 2181:2181 --network=host bitnami/zookeeper
1. Run Apache Kafka
   sudo docker run --rm -e KAFKA_CFG_LOG_CLEANUP_POLICY="compact, delete" -e KAFKA_CFG_ZOOKEEPER_CONNECT=localhost:2181 -e ALLOW_PLAINTEXT_LISTENER=yes -e KAFKA_CFG_AUTO_CREATE_TOPICS_ENABLE=true -p 9092:9092 --network=host bitnami/kafka:3