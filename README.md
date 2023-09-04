# Actix Load Testing between HTTP/1.1 and HTTP/2

## Overview

This project aims to benchmark the performance of two APIs: one running on HTTP/1.1 and another on HTTP/2. We use Actix Web for the APIs and run the services using Docker Compose. Load testing is performed automatically when the Docker Compose environment is started. What difference between HTTP and HTTPS.

## Getting Started

### Prerequisites

- Docker
- Docker Compose
- Rust (Optional, if you want to build the project manually)

### Setting up the Project

1. **Clone the Repository**

    ```bash
    git clone https://github.com/thanet-s/actix-http1-vs-http2.git
    ```

2. **Navigate to the Project Directory**

    ```bash
    cd actix-http1-vs-http2
    ```

3. **Build and Run Docker Containers**

    ```bash
    docker-compose up --build
    ```

The load tests will automatically be run against both APIs once the services are up.

## Architecture

- `http1`: Service running on Actix Web with HTTP/1.1 enabled
- `http2`: Service running on Actix Web with HTTP/2 enabled

## Built With

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

## Result on Macbook Air M1
```
rust-test-h2load-1  | starting benchmark...
rust-test-h2load-1  | spawning thread #0: 100 total client(s). 100000 total requests
rust-test-h2load-1  | Application protocol: http/1.1
rust-test-h2load-1  | progress: 10% done
rust-test-h2load-1  | progress: 20% done
rust-test-h2load-1  | progress: 30% done
rust-test-h2load-1  | progress: 40% done
rust-test-h2load-1  | progress: 50% done
rust-test-h2load-1  | progress: 60% done
rust-test-h2load-1  | progress: 70% done
rust-test-h2load-1  | progress: 80% done
rust-test-h2load-1  | progress: 90% done
rust-test-h2load-1  | progress: 100% done
rust-test-h2load-1  | 
rust-test-h2load-1  | finished in 234.41ms, 426599.32 req/s, 311.23MB/s
rust-test-h2load-1  | requests: 100000 total, 100000 started, 100000 done, 100000 succeeded, 0 failed, 0 errored, 0 timeout
rust-test-h2load-1  | status codes: 100001 2xx, 0 3xx, 0 4xx, 0 5xx
rust-test-h2load-1  | traffic: 72.96MB (76500000) total, 7.44MB (7800000) headers (space savings 0.00%), 62.56MB (65600000) data
rust-test-h2load-1  |                      min         max         mean         sd        +/- sd
rust-test-h2load-1  | time for request:    68.59ms    209.22ms    126.19ms     36.91ms    60.24%
rust-test-h2load-1  | time for connect:     3.45ms     40.37ms     29.90ms      8.50ms    71.00%
rust-test-h2load-1  | time to 1st byte:    72.60ms    214.64ms    147.50ms     37.10ms    56.00%
rust-test-h2load-1  | req/s           :    4337.90    13501.77     6772.62     2050.79    68.00%
rust-test-h2load-1  | 
rust-test-h2load-1  | starting benchmark...
rust-test-h2load-1  | spawning thread #0: 100 total client(s). 100000 total requests
rust-test-h2load-1  | TLS Protocol: TLSv1.2
rust-test-h2load-1  | Cipher: ECDHE-ECDSA-CHACHA20-POLY1305
rust-test-h2load-1  | Server Temp Key: X25519 253 bits
rust-test-h2load-1  | Application protocol: h2
rust-test-h2load-1  | progress: 10% done
rust-test-h2load-1  | progress: 20% done
rust-test-h2load-1  | progress: 30% done
rust-test-h2load-1  | progress: 40% done
rust-test-h2load-1  | progress: 50% done
rust-test-h2load-1  | progress: 60% done
rust-test-h2load-1  | progress: 70% done
rust-test-h2load-1  | progress: 80% done
rust-test-h2load-1  | progress: 90% done
rust-test-h2load-1  | progress: 100% done
rust-test-h2load-1  | 
rust-test-h2load-1  | finished in 1.21s, 82897.29 req/s, 54.55MB/s
rust-test-h2load-1  | requests: 100000 total, 100000 started, 100000 done, 100000 succeeded, 0 failed, 0 errored, 0 timeout
rust-test-h2load-1  | status codes: 100000 2xx, 0 3xx, 0 4xx, 0 5xx
rust-test-h2load-1  | traffic: 65.81MB (69005300) total, 882.32KB (903500) headers (space savings 89.73%), 62.37MB (65400000) data
rust-test-h2load-1  |                      min         max         mean         sd        +/- sd
rust-test-h2load-1  | time for request:   673.85ms       1.13s    916.65ms    128.70ms    57.00%
rust-test-h2load-1  | time for connect:    26.51ms     69.67ms     54.35ms     11.29ms    58.00%
rust-test-h2load-1  | time to 1st byte:    99.06ms       1.13s    806.73ms    250.56ms    79.00%
rust-test-h2load-1  | req/s           :     831.84     1250.32      878.96      111.42    90.00%
```

## Authors

- [thanet-s](https://github.com/thanet-s)
