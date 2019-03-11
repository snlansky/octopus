all: help
help:
	@echo
	@echo "帮助文档："
	@echo "  - make help              查看可用脚本"
	@echo "  - make protos            编译 Protobuf 协议文件"
	@echo "  - make native            编译原生可执行文件"
	@echo "  - make docker            编译 Docker 镜像"
	@echo "  - make start             本地启动所有服务"
	@echo "  - make stop              本地终止所有服务"
	@echo "  - make clean             清理可执行文件和 Docker 镜像"
	@echo

install:
	cargo install protobuf-codegen
	cargo install grpcio-compiler
	rustup component add rustfmt
start:
	docker-compose up -d
stop:
	docker-compose down
test:
	cargo test -- --nocapture
run:
	RUST_LOG=info ./target/debug/octopus --cluster=www.snlan.top:2181,www.snlan.top:2182,www.snlan.top:2183 --path=/dal_orm_release

fmt:
	cargo fmt