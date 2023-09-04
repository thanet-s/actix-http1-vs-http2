FROM debian:bookworm-slim

# Install the required package
RUN apt-get update \
    && apt-get install -y nghttp2-client \
    && apt-get clean

# Set the entrypoint to h2load
ENTRYPOINT ["h2load"]