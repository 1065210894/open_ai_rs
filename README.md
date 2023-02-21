# open_ai_service

#### 介绍
关于openai的web服务系统

### 项目的配置项逻辑
项目目前使用的配置来源于/src/config目录下的配置
config为默认配置
config-dev为本地环境配置
config-prod为生产配置
除了这三个配置文件以外还有可以额外装载当前系统环境下的配置文件
例如 config-dev [config] env_path = "E:\\env.toml"

### docker打包
docker 打包的逻辑是先用 nasqueron/rust-musl-builder:nightly 镜像打包
打包后可以在 alpine:latest镜像中部署大好的包

注意：因为可能涉及到当前系统的下的配置文件，在dockerfile中也要将需要的 env.toml文件 copy到镜像中