# Load Testing Project

## Overview

This project aims to benchmark the performance of two APIs: one running on HTTP/1.1 and another on HTTP/2. We use Actix Web for the APIs and run the services using Docker Compose. Load testing is performed automatically when the Docker Compose environment is started.

## Getting Started

### Prerequisites

- Docker
- Docker Compose
- Rust (Optional, if you want to build the project manually)

### Setting up the Project

1. **Clone the Repository**

    ```bash
    git clone https://github.com/yourusername/load-testing-project.git
    ```

2. **Navigate to the Project Directory**

    ```bash
    cd load-testing-project
    ```

3. **Build and Run Docker Containers**

    ```bash
    docker-compose up --build
    ```

The load tests will automatically be run against both APIs once the services are up.

## Architecture

- `api1`: Service running on Actix Web with HTTP/1.1 enabled
- `api2`: Service running on Actix Web with HTTP/2 enabled

## Built With

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## Authors

- [thanet-s](https://github.com/thanet-s)
