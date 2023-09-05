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
actix-http1-test-h2load  | 
actix-http1-test-h2load  | 
actix-http1-test-h2load  | finished in 12.45s, 803023.68 req/s, 585.85MB/s
actix-http1-test-h2load  | requests: 10000000 total, 10000000 started, 9994301 done, 9994301 succeeded, 5699 failed, 5699 errored, 0 timeout
actix-http1-test-h2load  | status codes: 9997105 2xx, 0 3xx, 0 4xx, 0 5xx
actix-http1-test-h2load  | traffic: 7.12GB (7645640265) total, 743.44MB (779555478) headers (space savings 0.00%), 6.11GB (6556261456) data
actix-http1-test-h2load  |                      min         max         mean         sd        +/- sd
actix-http1-test-h2load  | time for request:    14.42ms       2.17s    635.33ms    295.65ms    70.37%
actix-http1-test-h2load  | time for connect:      132us    118.37ms     52.85ms     34.52ms    60.00%
actix-http1-test-h2load  | time to 1st byte:   203.48ms    375.47ms    285.07ms     36.41ms    69.00%
actix-http1-test-h2load  | req/s           :    8041.11    22798.36    13353.84     1947.76    88.00%
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
actix-http2-test-h2load  | 
actix-http2-test-h2load  | 
actix-http2-test-h2load  | finished in 38.63s, 112663.69 req/s, 74.23MB/s
actix-http2-test-h2load  | requests: 10000000 total, 4982090 started, 4352090 done, 4352090 succeeded, 5647910 failed, 5647910 errored, 0 timeout
actix-http2-test-h2load  | status codes: 4362090 2xx, 0 3xx, 0 4xx, 0 5xx
actix-http2-test-h2load  | traffic: 2.80GB (3006860083) total, 37.45MB (39272315) headers (space savings 89.77%), 2.65GB (2849935146) data
actix-http2-test-h2load  |                      min         max         mean         sd        +/- sd
actix-http2-test-h2load  | time for request:   188.86ms      19.24s       7.72s       6.71s    61.52%
actix-http2-test-h2load  | time for connect:    67.93ms    339.59ms    222.43ms     72.52ms    62.00%
actix-http2-test-h2load  | time to 1st byte:   929.63ms      13.76s       8.35s       2.75s    82.00%
actix-http2-test-h2load  | req/s           :       0.00     2664.03     1323.98      990.97    62.00%
[+] Running 2/2
 ✔ Container actix-http2-test-h2load  Removed                                                                                                                                                                                                                                                                     0.0s 
 ✔ Container http2                    Removed                                                                                                                                                                                                                                                                     0.4s 
Both tests completed.
```

## Authors

- [thanet-s](https://github.com/thanet-s)
