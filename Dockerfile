# Use Rust Debian bookworm image as the build image
FROM rust:slim-bookworm AS build

# Switch work directory & copy backend sources
WORKDIR /usr/src/preck
COPY . .

# Install required dependencies & compile the project
RUN apt update
RUN apt install g++ mold -y
RUN RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build --release

# Copy the critical files into a pure Debian bookworm image
FROM debian:bookworm-slim
COPY --from=build /usr/src/preck/target/release/preck /usr/local/bin/preck

# Expose ports
EXPOSE 8081

# Run the web server
CMD ["preck"]