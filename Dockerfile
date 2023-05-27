# Use Rust Debian bookworm image as the build image
FROM rust:slim-bookworm AS build

# Use the new crates.io index protocol
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Switch work directory & copy backend sources
WORKDIR /usr/src/pid-checker
COPY . .

# Install required dependencies & compile the project
RUN apt update
RUN apt install g++ mold -y
RUN RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build --release

# Copy the critical files into a pure Debian bookworm image
FROM debian:bookworm-slim
COPY --from=build /usr/src/pid-checker/target/release/pidchecker /usr/local/bin/pidchecker

# Expose ports
EXPOSE 8081

# Run the web server
CMD ["pidchecker"]