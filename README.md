# open_ai_rs server

#### Introduction
About openai's web services system (some features are still under development)
auxm was used as a web service and the ability to add automatic assembly environment configuration was added.
Access to chatgpt-3 is provided externally through auxm

### project configuration item logic
The current configuration used by the project is derived from the configuration in the /src/config directory
config is the default configuration
config-dev Indicates the local environment configuration
config-prod indicates the production configuration
In addition to these three configuration files, you can also load additional configuration files for the current system environment
For example, config-dev [config] env_path = "E:\\env.toml"

### docker packing
The logic of docker packaging is to first package with the nasqueron/rust-musl-builder:nightly image
After packaging, you can deploy large packages in the alpine:latest image

Note: Because it may involve the configuration file of the current system, copy the required env.toml file into the image in the dockerfile as well

# open_ai_rs server

#### 介绍
关于openai的web服务系统 （部分功能还在开发中）
使用到了auxm作为web服务，并且添加自动装配环境配置的能力。
通过auxm对外提供访问chatgpt-3的能力

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
