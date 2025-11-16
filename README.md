1. Add a .env at the root of the infra project with DATABASE_URL=postgres://usr:pwd@localhost:5432/name-postgres changing for the name of your PostgreSQL database and usr/pwd.
2. cargo build
3. cargo run --bin divide_population_by_s

Prerequisites
. Docker container running with postgres image
. docker exec -it <container id> pqsl -U postgres (to connect to the database and check tables content)
. infra repo: diesel migration run (to create partitioned table and partitions)
. infra repo: diesel migration revert (to use the runtime partitions from 3.)  
