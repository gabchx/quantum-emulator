
FROM rust:1.72
WORKDIR /usr/src/quantum_emulator

# Create a new empty shell project to cache dependencies
COPY quantum_emulator/Cargo.toml quantum_emulator/Cargo.lock ./
COPY quantum_emulator/src ./src
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code into the container
COPY quantum_emulator/src ./src
#Build
RUN cargo build --release

# Expose port 8000
EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000


CMD ["./target/release/quantum_emulator"]