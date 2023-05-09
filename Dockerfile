FROM rust:slim-bullseye AS build

RUN apt-get update && apt-get install -y dpkg-dev && apt-get clean -y

WORKDIR /build

COPY wireguard-logd ./wireguard-logd
RUN mkdir -p wireguard-logd/usr/local/bin/

COPY Cargo.toml ./
COPY main.rs ./
RUN cargo build --release
RUN mv target/release/wglogd wireguard-logd/usr/local/bin/

RUN dpkg-deb --build --root-owner-group wireguard-logd
RUN dpkg-name wireguard-logd.deb

FROM scratch
COPY --from=build /build/*.deb /
