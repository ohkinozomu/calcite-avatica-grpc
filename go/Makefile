.PHONY: test

gen:
	@protoc --go_out=./ --go_opt=paths=source_relative \
    --go-grpc_out=./ --go-grpc_opt=paths=source_relative \
    --proto_path ../ avatica.proto common.proto requests.proto responses.proto