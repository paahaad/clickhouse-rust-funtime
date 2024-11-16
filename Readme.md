# clickhouse with single cmd

```
docker run -d --name ch-instance --ulimit nofile=262144:262144 clickhouse/clickhouse-server
```
