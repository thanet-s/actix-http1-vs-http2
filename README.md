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

3. **Run Loadtest**

    ```bash
    chmod +x run-loadtest.sh && ./run-loadtest.sh
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
Starting services defined in docker-compose-http1.yaml under project name actix-http1-test...
 ✔ Container http1                    Started                                                                                                                                                                                                                                                                     0.2s 
 ✔ Container actix-http1-test-h2load  Started                                                                                                                                                                                                                                                                     0.2s 
waiting for h2load for HTTP/1.1 to complete...
waiting for h2load for HTTP/1.1 to complete...
waiting for h2load for HTTP/1.1 to complete...
waiting for h2load for HTTP/1.1 to complete...
h2load for HTTP/1.1 completed.
Fetching logs for h2load...
actix-http1-test-h2load  | starting benchmark...
actix-http1-test-h2load  | spawning thread #0: 100 total client(s). 10000000 total requests
actix-http1-test-h2load  | Application protocol: http/1.1
actix-http1-test-h2load  | progress: 10% done
actix-http1-test-h2load  | progress: 20% done
actix-http1-test-h2load  | progress: 30% done
actix-http1-test-h2load  | progress: 40% done
actix-http1-test-h2load  | progress: 50% done
actix-http1-test-h2load  | progress: 60% done
actix-http1-test-h2load  | progress: 70% done
actix-http1-test-h2load  | progress: 80% done
actix-http1-test-h2load  | progress: 90% done
actix-http1-test-h2load  | progress: 100% done
actix-http1-test-h2load  | 
actix-http1-test-h2load  | 
actix-http1-test-h2load  | finished in 5.21s, 1918803.53 req/s, 1.37GB/s
actix-http1-test-h2load  | requests: 10000000 total, 10000000 started, 10000000 done, 10000000 succeeded, 0 failed, 0 errored, 0 timeout
actix-http1-test-h2load  | status codes: 10000000 2xx, 0 3xx, 0 4xx, 0 5xx
actix-http1-test-h2load  | traffic: 7.12GB (7650000000) total, 743.87MB (780000000) headers (space savings 0.00%), 6.11GB (6560000000) data
actix-http1-test-h2load  |                      min         max         mean         sd        +/- sd
actix-http1-test-h2load  | time for request:     1.70ms    242.95ms     52.71ms     18.30ms    91.23%
actix-http1-test-h2load  | time for connect:      148us     19.58ms      8.03ms      5.47ms    62.00%
actix-http1-test-h2load  | time to 1st byte:    75.64ms    210.23ms    131.98ms     30.96ms    68.00%
actix-http1-test-h2load  | req/s           :   19210.89    20318.67    19504.44      176.24    80.00%
[+] Running 2/2
 ✔ Container actix-http1-test-h2load  Removed                                                                                                                                                                                                                                                                     0.0s 
 ✔ Container http1                    Removed                                                                                                                                                                                                                                                                     0.4s 
Starting services defined in docker-compose-http2.yaml under project name actix-http2-test...                                                                                                                                                                                                                                              0.0s
[+] Running 2/2
 ✔ Container http2                    Started                                                                                                                                                                                                                                                                     0.1s 
 ✔ Container actix-http2-test-h2load  Started                                                                                                                                                                                                                                                                     0.1s 
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
waiting for h2load for HTTP/2 to complete...
h2load for HTTP/2 completed.
Fetching logs for h2load...
actix-http2-test-h2load  | starting benchmark...
actix-http2-test-h2load  | spawning thread #0: 100 total client(s). 10000000 total requests
actix-http2-test-h2load  | TLS Protocol: TLSv1.2
actix-http2-test-h2load  | Cipher: ECDHE-ECDSA-CHACHA20-POLY1305
actix-http2-test-h2load  | Server Temp Key: X25519 253 bits
actix-http2-test-h2load  | Application protocol: h2
actix-http2-test-h2load  | progress: 10% done
actix-http2-test-h2load  | progress: 20% done
actix-http2-test-h2load  | progress: 30% done
actix-http2-test-h2load  | progress: 40% done
actix-http2-test-h2load  | progress: 50% done
actix-http2-test-h2load  | progress: 60% done
actix-http2-test-h2load  | progress: 70% done
actix-http2-test-h2load  | progress: 80% done
actix-http2-test-h2load  | progress: 90% done
actix-http2-test-h2load  | progress: 100% done
actix-http2-test-h2load  | 
actix-http2-test-h2load  | 
actix-http2-test-h2load  | finished in 52.66s, 189881.40 req/s, 124.95MB/s
actix-http2-test-h2load  | requests: 10000000 total, 10000000 started, 10000000 done, 10000000 succeeded, 0 failed, 0 errored, 0 timeout
actix-http2-test-h2load  | status codes: 10000000 2xx, 0 3xx, 0 4xx, 0 5xx
actix-http2-test-h2load  | traffic: 6.43GB (6900141532) total, 85.95MB (90122732) headers (space savings 89.76%), 6.09GB (6540000000) data
actix-http2-test-h2load  |                      min         max         mean         sd        +/- sd
actix-http2-test-h2load  | time for request:   127.04ms       1.33s    292.44ms    151.65ms    96.05%
actix-http2-test-h2load  | time for connect:    71.02ms     95.50ms     81.34ms      6.65ms    62.00%
actix-http2-test-h2load  | time to 1st byte:   177.20ms       1.28s    977.36ms    275.21ms    84.00%
actix-http2-test-h2load  | req/s           :    1898.98     1921.51     1902.11        4.48    84.00%
[+] Running 2/2
 ✔ Container actix-http2-test-h2load  Removed                                                                                                                                                                                                                                                                     0.0s 
 ✔ Container http2                    Removed                                                                                                                                                                                                                                                                     0.4s 
Both tests completed.
```

## Authors

- [thanet-s](https://github.com/thanet-s)
