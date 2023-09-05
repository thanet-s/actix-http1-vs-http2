#!/bin/bash

# Exit script on error
set -e

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo "Docker does not seem to be running, start it first and then run this script."
    exit 1
fi

echo "Building Docker images..."
docker-compose -f docker-compose-http1.yaml build --pull
docker-compose -f docker-compose-http2.yaml build --pull
docker-compose -f docker-compose-http2-haproxy.yaml build --pull
echo "Docker images built."

# Function to run Docker Compose
run_docker_compose() {
    project_name=$2
    echo "Starting services defined in $1 under project name $project_name..."
    docker-compose -f $1 -p $project_name up -d
}

# Function to get the status of h2load container
get_h2load_status() {
    project_name=$1
    docker ps -a --filter "name=${project_name}-h2load" --format "{{.Status}}"
}

# Function to show logs of h2load container
show_h2load_logs() {
    echo "Fetching logs for h2load..."
    docker-compose -p $1 logs h2load
}

# Function to stop all containers defined in a docker-compose file
stop_docker_compose() {
    project_name=$2
    docker-compose -f $1 -p $project_name down
}

# Run HTTP/1.1 service
run_docker_compose "docker-compose-http1.yaml" "actix-http1-test"

# Wait for h2load for HTTP/1.1 to complete
while true; do
    h2load_status=$(get_h2load_status "actix-http1-test")
    echo "waiting for h2load for HTTP/1.1 to complete..."
    if [[ $h2load_status == Exited* ]]; then
        echo "h2load for HTTP/1.1 completed."
        show_h2load_logs "actix-http1-test"
        stop_docker_compose "docker-compose-http1.yaml" "actix-http1-test"
        break
    fi
    sleep 5
done

# Run HTTP/2 service
run_docker_compose "docker-compose-http2.yaml" "actix-http2-test"

# Wait for h2load for HTTP/2 to complete
while true; do
    h2load_status=$(get_h2load_status "actix-http2-test")
    echo "waiting for h2load for HTTP/2 to complete..."
    if [[ $h2load_status == Exited* ]]; then
        echo "h2load for HTTP/2 completed."
        show_h2load_logs "actix-http2-test"
        stop_docker_compose "docker-compose-http2.yaml" "actix-http2-test"
        break
    fi
    sleep 5
done

# Run HTTP/2 HAPROXY service
run_docker_compose "docker-compose-http2-haproxy.yaml" "actix-http2-haproxy-test"

# Wait for h2load for HTTP/2 to complete
while true; do
    h2load_status=$(get_h2load_status "actix-http2-haproxy-test")
    echo "waiting for h2load for HTTP/2 haproxy to complete..."
    if [[ $h2load_status == Exited* ]]; then
        echo "h2load for HTTP/2 haproxy completed."
        show_h2load_logs "actix-http2-haproxy-test"
        stop_docker_compose "docker-compose-http2-haproxy.yaml" "actix-http2-haproxy-test"
        break
    fi
    sleep 5
done

echo "Both tests completed."
