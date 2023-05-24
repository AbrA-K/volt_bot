FROM surrealdb/surrealdb:latest

WORKDIR /app

EXPOSE 8000

CMD ["start", "--user", "root", "--pass", "root", "--bind", "0.0.0.0:8000", "file://data/srdb.db"]
