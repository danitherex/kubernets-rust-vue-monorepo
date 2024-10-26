FROM rust:1.77.2-slim-bullseye as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/simple-rust-backend /build-out/

FROM ubuntu:24.10

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/simple-rust-backend /

CMD /simple-rust-backend