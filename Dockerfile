# Use Python as the base image
FROM python:3.12-bullseye AS base

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    ~/.cargo/bin/rustup default stable && \
    ~/.cargo/bin/rustup update && \
    ~/.cargo/bin/rustc --version

# Set the working directory
WORKDIR /usr/src/app

## Rust setup
COPY quantum_emulator/Cargo.toml quantum_emulator/Cargo.lock ./

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

## Python setup
COPY quantum_emulator/pyproject.toml quantum_emulator/poetry.lock ./
RUN curl -sSL https://install.python-poetry.org | python3 -

# Copy the source code into the container
COPY quantum_emulator/src ./src

# Install dependencies
RUN ~/.cargo/bin/cargo build --release
RUN ~/.local/bin/poetry install --only main

# Expose ports
EXPOSE 8000
EXPOSE 8001

# Copy the entrypoint script and set permissions
COPY run.sh /usr/src/app/
RUN chmod +x /usr/src/app/run.sh

# Run the entrypoint script
ENTRYPOINT ["./run.sh"]
CMD ["run"]
