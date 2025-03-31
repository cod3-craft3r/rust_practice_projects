FROM ubuntu:24.04
LABEL maintainer="Archit Sharma <awsmarchies@gmail.com>" \
      version="1.0" \
      description="A Docker image for setting up a Rust development environment"

# Install essential packages
RUN apt-get -y update && apt-get upgrade -y && \
    apt-get install -y build-essential curl git && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup update

# Set the working directory
WORKDIR /rust-dev

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Set the default command to run when starting the container
CMD ["/bin/bash"]
