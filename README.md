# ai-prop

`ai-prop` is a Rust-based statistical microservice that exposes statistical calculation logic via both REST (`axum`) and gRPC (`tonic`) APIs. It interacts with a PostgreSQL database via the `ai-infra` dependency for certain operations.

## APIs

The service exposes the following endpoints (available on both REST and gRPC):
- `/calculate_proportions`
- `/calculate_pooled_estimate`
- `/calculate_z_statistics`

### Ports
- **REST (Axum):** Port 3000 (by default) or configured via the `PORT` environment variable.
- **gRPC (Tonic):** Port 50051 (by default)

### Live Deployment
The REST API is currently deployed on Render and is accessible at:
[https://ai-prop-service.onrender.com](https://ai-prop-service.onrender.com)

## Binaries

This project contains two primary binaries:
- `server`: The main microservice providing the REST and gRPC endpoints (run by default with `cargo run`).
- `divide_population_by_s`: A utility binary for dividing population data into database partitions. (run with `cargo run --bin divide_population_by_s`).

## Local Development Prerequisites

- Docker container running with the `postgres` image
- To connect to the database and check tables content:
  `docker exec -it <container id> psql -U postgres`

### Infra Setup (via the `ai-infra` repo)
Run the following commands in the `ai-infra` repository:
- `diesel migration run` (to create the partitioned table and partitions)
- `diesel migration revert` (to use the runtime partitions from step 3 below)

## Getting Started

1. Add a `.env` at the root of the infra project with:
   `DATABASE_URL=postgres://usr:pwd@localhost:5432/name-postgres`
   *(change for the name of your PostgreSQL database and usr/pwd)*
2. `cargo build`
3. `cargo run --bin divide_population_by_s`
