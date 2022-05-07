FROM rust:1.60 as builder

WORKDIR /usr/src

RUN USER=root cargo new --bin api
WORKDIR /usr/src/api
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN USER=root cargo new --lib entity
RUN USER=root cargo new --lib migration

WORKDIR /usr/src/api/entity
COPY ./entity/Cargo.toml ./Cargo.toml

WORKDIR /usr/src/api/migration
COPY ./migration/Cargo.toml ./Cargo.toml

WORKDIR /usr/src/api
RUN cargo build --release

RUN rm src/*.rs
RUN rm entity/src/*.rs
RUN rm migration/src/*.rs

ADD . ./

RUN rm ./target/release/deps/api*
RUN rm ./target/release/deps/entity*
RUN rm ./target/release/deps/libentity*
RUN rm ./target/release/deps/migration*
RUN rm ./target/release/deps/libmigration*

RUN cargo build --release


FROM debian:buster-slim
ARG BIND_PORT=8000
ARG APP=/usr/src/api

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE ${BIND_PORT}

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /usr/src/api/target/release/api ${APP}/api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./api"]