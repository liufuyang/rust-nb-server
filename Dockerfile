FROM rustlang/rust:nightly-slim as build 

RUN set -eux;
    # apt-get update; \
    # apt-get install -y --no-install-recommends \
    #     pkg-config libssl-dev libcurl4-gnutls-dev libz-dev

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --bin backend
#  && \
#   USER=root cargo new --lib oauth2-rs

# copy over your manifests
COPY ./Cargo.lock /backend/Cargo.lock
COPY ./Cargo.toml /backend/Cargo.toml

# this build step will allow cargo to download and compile all the dependencies
# this process might take a long time
WORKDIR /backend
RUN cargo build --release
RUN rm ./src/*.rs

# copy source code and build again
COPY ./src        /backend/src
RUN touch /backend/src/*.rs 
RUN cargo build --release

# #####################################################
# our final image, which has no build tools added on
FROM debian:stretch-slim

# copy the build artifact from the build stage
WORKDIR /
COPY --from=build /backend/target/release/rust-nb-server .

# COPY ./Rocket.toml .

# set the startup command to run your binary
CMD ["./rust-nb-server"]