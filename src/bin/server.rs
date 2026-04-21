use ai_prop::{
    PopulationData, calculate_pooled_estimate, calculate_proportions, calculate_z_statistics,
};
use axum::{Router, extract::Json, routing::post};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::{Request, Response, Status, transport::Server};

pub mod ai_prop_grpc {
    tonic::include_proto!("ai_prop");
}

use ai_prop_grpc::ai_prop_service_server::{AiPropService, AiPropServiceServer};
use ai_prop_grpc::{
    CalculatePooledEstimateRequest, CalculatePooledEstimateResponse, CalculateProportionsRequest,
    CalculateProportionsResponse, CalculateZStatisticsRequest, CalculateZStatisticsResponse,
};

// gRPC Service Implementation
#[derive(Debug, Default)]
pub struct MyAiPropService {}

#[tonic::async_trait]
impl AiPropService for MyAiPropService {
    async fn calculate_proportions(
        &self,
        request: Request<CalculateProportionsRequest>,
    ) -> Result<Response<CalculateProportionsResponse>, Status> {
        let req = request.into_inner();

        let overall_req = req
            .overall
            .ok_or_else(|| Status::invalid_argument("overall missing"))?;
        let group1_req = req
            .group1
            .ok_or_else(|| Status::invalid_argument("group1 missing"))?;
        let group2_req = req
            .group2
            .ok_or_else(|| Status::invalid_argument("group2 missing"))?;

        let overall = PopulationData {
            m: overall_req.m,
            n: overall_req.n,
        };
        let group1 = PopulationData {
            m: group1_req.m,
            n: group1_req.n,
        };
        let group2 = PopulationData {
            m: group2_req.m,
            n: group2_req.n,
        };

        let (p_population, p1, p2) = calculate_proportions(overall, group1, group2);

        Ok(Response::new(CalculateProportionsResponse {
            p_population,
            p1,
            p2,
        }))
    }

    async fn calculate_pooled_estimate(
        &self,
        request: Request<CalculatePooledEstimateRequest>,
    ) -> Result<Response<CalculatePooledEstimateResponse>, Status> {
        let req = request.into_inner();
        let p = calculate_pooled_estimate(req.n1, req.n2, req.p1, req.p2);
        Ok(Response::new(CalculatePooledEstimateResponse { p }))
    }

    async fn calculate_z_statistics(
        &self,
        request: Request<CalculateZStatisticsRequest>,
    ) -> Result<Response<CalculateZStatisticsResponse>, Status> {
        let req = request.into_inner();
        let z = calculate_z_statistics(req.n1, req.n2, req.p1, req.p2, req.pooled_estimate);
        Ok(Response::new(CalculateZStatisticsResponse { z }))
    }
}

// REST Types
#[derive(Deserialize)]
struct RestPopulationData {
    m: f32,
    n: f32,
}

#[derive(Deserialize)]
struct RestCalculateProportionsRequest {
    overall: RestPopulationData,
    group1: RestPopulationData,
    group2: RestPopulationData,
}

#[derive(Serialize)]
struct RestCalculateProportionsResponse {
    p_population: f32,
    p1: f32,
    p2: f32,
}

#[derive(Deserialize)]
struct RestCalculatePooledEstimateRequest {
    n1: f32,
    n2: f32,
    p1: f32,
    p2: f32,
}

#[derive(Serialize)]
struct RestCalculatePooledEstimateResponse {
    p: f32,
}

#[derive(Deserialize)]
struct RestCalculateZStatisticsRequest {
    n1: f32,
    n2: f32,
    p1: f32,
    p2: f32,
    pooled_estimate: f32,
}

#[derive(Serialize)]
struct RestCalculateZStatisticsResponse {
    z: f32,
}

// REST Handlers
async fn rest_calculate_proportions(
    Json(payload): Json<RestCalculateProportionsRequest>,
) -> Json<RestCalculateProportionsResponse> {
    let overall = PopulationData {
        m: payload.overall.m,
        n: payload.overall.n,
    };
    let group1 = PopulationData {
        m: payload.group1.m,
        n: payload.group1.n,
    };
    let group2 = PopulationData {
        m: payload.group2.m,
        n: payload.group2.n,
    };

    let (p_population, p1, p2) = calculate_proportions(overall, group1, group2);

    Json(RestCalculateProportionsResponse {
        p_population,
        p1,
        p2,
    })
}

async fn rest_calculate_pooled_estimate(
    Json(payload): Json<RestCalculatePooledEstimateRequest>,
) -> Json<RestCalculatePooledEstimateResponse> {
    let p = calculate_pooled_estimate(payload.n1, payload.n2, payload.p1, payload.p2);
    Json(RestCalculatePooledEstimateResponse { p })
}

async fn rest_calculate_z_statistics(
    Json(payload): Json<RestCalculateZStatisticsRequest>,
) -> Json<RestCalculateZStatisticsResponse> {
    let z = calculate_z_statistics(
        payload.n1,
        payload.n2,
        payload.p1,
        payload.p2,
        payload.pooled_estimate,
    );
    Json(RestCalculateZStatisticsResponse { z })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start gRPC server in a separate task
    let grpc_addr = "0.0.0.0:50051".parse()?;
    let ai_prop_service = MyAiPropService::default();

    println!("Starting gRPC server on {}", grpc_addr);
    let grpc_future = Server::builder()
        .add_service(AiPropServiceServer::new(ai_prop_service))
        .serve(grpc_addr);

    // Start REST server
    let app = Router::new()
        .route("/calculate_proportions", post(rest_calculate_proportions))
        .route(
            "/calculate_pooled_estimate",
            post(rest_calculate_pooled_estimate),
        )
        .route("/calculate_z_statistics", post(rest_calculate_z_statistics));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;
    let rest_addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Starting REST server on {}", rest_addr);
    let listener = TcpListener::bind(rest_addr).await?;
    let rest_future = axum::serve(listener, app);

    // Wait for both to finish (which they shouldn't unless interrupted)
    tokio::try_join!(
        async {
            grpc_future
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        },
        async {
            rest_future
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
    )?;

    Ok(())
}
