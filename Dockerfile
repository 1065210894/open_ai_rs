FROM nasqueron/rust-musl-builder:nightly as builder

RUN USER=root cargo new --bin open_ai_service
WORKDIR ./open_ai_service
COPY ./Cargo.toml ./Cargo.toml
COPY ./alpine/config  /opt/rust/cargo

RUN CARGO_HTTP_MULTIPLEXING=false cargo fetch
RUN cargo build --release

ADD . ./

RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8080

ENV TZ=Etc/UTC \
  APP_USER=rustuser

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata \
  && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/open_ai_service/target/x86_64-unknown-linux-musl/release/open_ai_service ${APP}/open_ai_service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./open_ai_service"]