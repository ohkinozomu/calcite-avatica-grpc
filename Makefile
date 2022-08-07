.PHONY: test

test:
	@go test ./...

fmt:
	@go fmt ./...

gen:
	@protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    avatica.proto common.proto requests.proto responses.proto