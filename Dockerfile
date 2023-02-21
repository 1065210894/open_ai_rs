FROM nasqueron/rust-musl-builder:nightly as builder
RUN USER=root  \
WORKDIR ./open_ai_service
# 拷贝项目需要的额外环境配置文件
COPY ./env.toml ./env.toml
ADD . ./
RUN cargo build --release

FROM alpine:latest

# 项目名称
ARG APP_NAME=open_ai_service
# 项目在容器中的路径
ARG APP_PATH=/usr/src/$APP_NAME

EXPOSE 8080

ENV TZ=Etc/UTC \
  APP_USER=root

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata \
  && rm -rf /var/cache/apk/*

# 将上面镜像打好的包拷贝过来
COPY --from=builder /home/rust/src/$APP_NAME/target/x86_64-unknown-linux-musl/release/$APP_NAME ${APP_PATH}/

# 拷贝项目环境变量文件
COPY --from=builder /home/rust/src/$APP_NAME/env.toml ${APP_PATH}/

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP_PATH}

CMD ["./$APP_NAME"]