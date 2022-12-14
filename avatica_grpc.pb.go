// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.21.4
// source: avatica.proto

package calcite_avatica_grpc

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// AvaticaClient is the client API for Avatica service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type AvaticaClient interface {
	Catalogs(ctx context.Context, in *CatalogsRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	Schemas(ctx context.Context, in *SchemasRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	Tables(ctx context.Context, in *TablesRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	TableTypes(ctx context.Context, in *TableTypesRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	TypeInfo(ctx context.Context, in *TypeInfoRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	Columns(ctx context.Context, in *ColumnsRequest, opts ...grpc.CallOption) (*ResultSetResponse, error)
	Prepare(ctx context.Context, in *PrepareRequest, opts ...grpc.CallOption) (*PrepareResponse, error)
	Execute(ctx context.Context, in *ExecuteRequest, opts ...grpc.CallOption) (*ExecuteResponse, error)
	PrepareAndExecute(ctx context.Context, in *PrepareAndExecuteRequest, opts ...grpc.CallOption) (*ExecuteResponse, error)
	SyncResults(ctx context.Context, in *SyncResultsRequest, opts ...grpc.CallOption) (*SyncResultsResponse, error)
	Fetch(ctx context.Context, in *FetchRequest, opts ...grpc.CallOption) (*FetchResponse, error)
	CreateStatement(ctx context.Context, in *CreateStatementRequest, opts ...grpc.CallOption) (*CreateStatementResponse, error)
	CloseStatement(ctx context.Context, in *CloseStatementRequest, opts ...grpc.CallOption) (*CloseStatementResponse, error)
	OpenConnection(ctx context.Context, in *OpenConnectionRequest, opts ...grpc.CallOption) (*OpenConnectionResponse, error)
	CloseConnection(ctx context.Context, in *CloseConnectionRequest, opts ...grpc.CallOption) (*CloseConnectionResponse, error)
	ConnectionSync(ctx context.Context, in *ConnectionSyncRequest, opts ...grpc.CallOption) (*ConnectionSyncResponse, error)
	DatabaseProperty(ctx context.Context, in *DatabasePropertyRequest, opts ...grpc.CallOption) (*DatabasePropertyResponse, error)
	Commit(ctx context.Context, in *CommitRequest, opts ...grpc.CallOption) (*CommitResponse, error)
	Rollback(ctx context.Context, in *RollbackRequest, opts ...grpc.CallOption) (*RollbackResponse, error)
	PrepareAndExecuteBatch(ctx context.Context, in *PrepareAndExecuteBatchRequest, opts ...grpc.CallOption) (*ExecuteBatchResponse, error)
	ExecuteBatch(ctx context.Context, in *ExecuteBatchRequest, opts ...grpc.CallOption) (*ExecuteBatchResponse, error)
}

type avaticaClient struct {
	cc grpc.ClientConnInterface
}

func NewAvaticaClient(cc grpc.ClientConnInterface) AvaticaClient {
	return &avaticaClient{cc}
}

func (c *avaticaClient) Catalogs(ctx context.Context, in *CatalogsRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Catalogs", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Schemas(ctx context.Context, in *SchemasRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Schemas", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Tables(ctx context.Context, in *TablesRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Tables", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) TableTypes(ctx context.Context, in *TableTypesRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/TableTypes", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) TypeInfo(ctx context.Context, in *TypeInfoRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/TypeInfo", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Columns(ctx context.Context, in *ColumnsRequest, opts ...grpc.CallOption) (*ResultSetResponse, error) {
	out := new(ResultSetResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Columns", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Prepare(ctx context.Context, in *PrepareRequest, opts ...grpc.CallOption) (*PrepareResponse, error) {
	out := new(PrepareResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Prepare", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Execute(ctx context.Context, in *ExecuteRequest, opts ...grpc.CallOption) (*ExecuteResponse, error) {
	out := new(ExecuteResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Execute", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) PrepareAndExecute(ctx context.Context, in *PrepareAndExecuteRequest, opts ...grpc.CallOption) (*ExecuteResponse, error) {
	out := new(ExecuteResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/PrepareAndExecute", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) SyncResults(ctx context.Context, in *SyncResultsRequest, opts ...grpc.CallOption) (*SyncResultsResponse, error) {
	out := new(SyncResultsResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/SyncResults", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Fetch(ctx context.Context, in *FetchRequest, opts ...grpc.CallOption) (*FetchResponse, error) {
	out := new(FetchResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Fetch", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) CreateStatement(ctx context.Context, in *CreateStatementRequest, opts ...grpc.CallOption) (*CreateStatementResponse, error) {
	out := new(CreateStatementResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/CreateStatement", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) CloseStatement(ctx context.Context, in *CloseStatementRequest, opts ...grpc.CallOption) (*CloseStatementResponse, error) {
	out := new(CloseStatementResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/CloseStatement", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) OpenConnection(ctx context.Context, in *OpenConnectionRequest, opts ...grpc.CallOption) (*OpenConnectionResponse, error) {
	out := new(OpenConnectionResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/OpenConnection", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) CloseConnection(ctx context.Context, in *CloseConnectionRequest, opts ...grpc.CallOption) (*CloseConnectionResponse, error) {
	out := new(CloseConnectionResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/CloseConnection", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) ConnectionSync(ctx context.Context, in *ConnectionSyncRequest, opts ...grpc.CallOption) (*ConnectionSyncResponse, error) {
	out := new(ConnectionSyncResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/ConnectionSync", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) DatabaseProperty(ctx context.Context, in *DatabasePropertyRequest, opts ...grpc.CallOption) (*DatabasePropertyResponse, error) {
	out := new(DatabasePropertyResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/DatabaseProperty", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Commit(ctx context.Context, in *CommitRequest, opts ...grpc.CallOption) (*CommitResponse, error) {
	out := new(CommitResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Commit", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) Rollback(ctx context.Context, in *RollbackRequest, opts ...grpc.CallOption) (*RollbackResponse, error) {
	out := new(RollbackResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/Rollback", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) PrepareAndExecuteBatch(ctx context.Context, in *PrepareAndExecuteBatchRequest, opts ...grpc.CallOption) (*ExecuteBatchResponse, error) {
	out := new(ExecuteBatchResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/PrepareAndExecuteBatch", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *avaticaClient) ExecuteBatch(ctx context.Context, in *ExecuteBatchRequest, opts ...grpc.CallOption) (*ExecuteBatchResponse, error) {
	out := new(ExecuteBatchResponse)
	err := c.cc.Invoke(ctx, "/avatica_grpc.Avatica/ExecuteBatch", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// AvaticaServer is the server API for Avatica service.
// All implementations must embed UnimplementedAvaticaServer
// for forward compatibility
type AvaticaServer interface {
	Catalogs(context.Context, *CatalogsRequest) (*ResultSetResponse, error)
	Schemas(context.Context, *SchemasRequest) (*ResultSetResponse, error)
	Tables(context.Context, *TablesRequest) (*ResultSetResponse, error)
	TableTypes(context.Context, *TableTypesRequest) (*ResultSetResponse, error)
	TypeInfo(context.Context, *TypeInfoRequest) (*ResultSetResponse, error)
	Columns(context.Context, *ColumnsRequest) (*ResultSetResponse, error)
	Prepare(context.Context, *PrepareRequest) (*PrepareResponse, error)
	Execute(context.Context, *ExecuteRequest) (*ExecuteResponse, error)
	PrepareAndExecute(context.Context, *PrepareAndExecuteRequest) (*ExecuteResponse, error)
	SyncResults(context.Context, *SyncResultsRequest) (*SyncResultsResponse, error)
	Fetch(context.Context, *FetchRequest) (*FetchResponse, error)
	CreateStatement(context.Context, *CreateStatementRequest) (*CreateStatementResponse, error)
	CloseStatement(context.Context, *CloseStatementRequest) (*CloseStatementResponse, error)
	OpenConnection(context.Context, *OpenConnectionRequest) (*OpenConnectionResponse, error)
	CloseConnection(context.Context, *CloseConnectionRequest) (*CloseConnectionResponse, error)
	ConnectionSync(context.Context, *ConnectionSyncRequest) (*ConnectionSyncResponse, error)
	DatabaseProperty(context.Context, *DatabasePropertyRequest) (*DatabasePropertyResponse, error)
	Commit(context.Context, *CommitRequest) (*CommitResponse, error)
	Rollback(context.Context, *RollbackRequest) (*RollbackResponse, error)
	PrepareAndExecuteBatch(context.Context, *PrepareAndExecuteBatchRequest) (*ExecuteBatchResponse, error)
	ExecuteBatch(context.Context, *ExecuteBatchRequest) (*ExecuteBatchResponse, error)
	mustEmbedUnimplementedAvaticaServer()
}

// UnimplementedAvaticaServer must be embedded to have forward compatible implementations.
type UnimplementedAvaticaServer struct {
}

func (UnimplementedAvaticaServer) Catalogs(context.Context, *CatalogsRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Catalogs not implemented")
}
func (UnimplementedAvaticaServer) Schemas(context.Context, *SchemasRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Schemas not implemented")
}
func (UnimplementedAvaticaServer) Tables(context.Context, *TablesRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Tables not implemented")
}
func (UnimplementedAvaticaServer) TableTypes(context.Context, *TableTypesRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method TableTypes not implemented")
}
func (UnimplementedAvaticaServer) TypeInfo(context.Context, *TypeInfoRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method TypeInfo not implemented")
}
func (UnimplementedAvaticaServer) Columns(context.Context, *ColumnsRequest) (*ResultSetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Columns not implemented")
}
func (UnimplementedAvaticaServer) Prepare(context.Context, *PrepareRequest) (*PrepareResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Prepare not implemented")
}
func (UnimplementedAvaticaServer) Execute(context.Context, *ExecuteRequest) (*ExecuteResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Execute not implemented")
}
func (UnimplementedAvaticaServer) PrepareAndExecute(context.Context, *PrepareAndExecuteRequest) (*ExecuteResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PrepareAndExecute not implemented")
}
func (UnimplementedAvaticaServer) SyncResults(context.Context, *SyncResultsRequest) (*SyncResultsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SyncResults not implemented")
}
func (UnimplementedAvaticaServer) Fetch(context.Context, *FetchRequest) (*FetchResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Fetch not implemented")
}
func (UnimplementedAvaticaServer) CreateStatement(context.Context, *CreateStatementRequest) (*CreateStatementResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateStatement not implemented")
}
func (UnimplementedAvaticaServer) CloseStatement(context.Context, *CloseStatementRequest) (*CloseStatementResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CloseStatement not implemented")
}
func (UnimplementedAvaticaServer) OpenConnection(context.Context, *OpenConnectionRequest) (*OpenConnectionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method OpenConnection not implemented")
}
func (UnimplementedAvaticaServer) CloseConnection(context.Context, *CloseConnectionRequest) (*CloseConnectionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CloseConnection not implemented")
}
func (UnimplementedAvaticaServer) ConnectionSync(context.Context, *ConnectionSyncRequest) (*ConnectionSyncResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ConnectionSync not implemented")
}
func (UnimplementedAvaticaServer) DatabaseProperty(context.Context, *DatabasePropertyRequest) (*DatabasePropertyResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DatabaseProperty not implemented")
}
func (UnimplementedAvaticaServer) Commit(context.Context, *CommitRequest) (*CommitResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Commit not implemented")
}
func (UnimplementedAvaticaServer) Rollback(context.Context, *RollbackRequest) (*RollbackResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Rollback not implemented")
}
func (UnimplementedAvaticaServer) PrepareAndExecuteBatch(context.Context, *PrepareAndExecuteBatchRequest) (*ExecuteBatchResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PrepareAndExecuteBatch not implemented")
}
func (UnimplementedAvaticaServer) ExecuteBatch(context.Context, *ExecuteBatchRequest) (*ExecuteBatchResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ExecuteBatch not implemented")
}
func (UnimplementedAvaticaServer) mustEmbedUnimplementedAvaticaServer() {}

// UnsafeAvaticaServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to AvaticaServer will
// result in compilation errors.
type UnsafeAvaticaServer interface {
	mustEmbedUnimplementedAvaticaServer()
}

func RegisterAvaticaServer(s grpc.ServiceRegistrar, srv AvaticaServer) {
	s.RegisterService(&Avatica_ServiceDesc, srv)
}

func _Avatica_Catalogs_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CatalogsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Catalogs(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Catalogs",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Catalogs(ctx, req.(*CatalogsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Schemas_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SchemasRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Schemas(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Schemas",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Schemas(ctx, req.(*SchemasRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Tables_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(TablesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Tables(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Tables",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Tables(ctx, req.(*TablesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_TableTypes_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(TableTypesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).TableTypes(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/TableTypes",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).TableTypes(ctx, req.(*TableTypesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_TypeInfo_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(TypeInfoRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).TypeInfo(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/TypeInfo",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).TypeInfo(ctx, req.(*TypeInfoRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Columns_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ColumnsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Columns(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Columns",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Columns(ctx, req.(*ColumnsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Prepare_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PrepareRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Prepare(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Prepare",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Prepare(ctx, req.(*PrepareRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Execute_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ExecuteRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Execute(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Execute",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Execute(ctx, req.(*ExecuteRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_PrepareAndExecute_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PrepareAndExecuteRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).PrepareAndExecute(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/PrepareAndExecute",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).PrepareAndExecute(ctx, req.(*PrepareAndExecuteRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_SyncResults_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SyncResultsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).SyncResults(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/SyncResults",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).SyncResults(ctx, req.(*SyncResultsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Fetch_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(FetchRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Fetch(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Fetch",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Fetch(ctx, req.(*FetchRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_CreateStatement_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateStatementRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).CreateStatement(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/CreateStatement",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).CreateStatement(ctx, req.(*CreateStatementRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_CloseStatement_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CloseStatementRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).CloseStatement(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/CloseStatement",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).CloseStatement(ctx, req.(*CloseStatementRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_OpenConnection_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(OpenConnectionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).OpenConnection(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/OpenConnection",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).OpenConnection(ctx, req.(*OpenConnectionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_CloseConnection_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CloseConnectionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).CloseConnection(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/CloseConnection",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).CloseConnection(ctx, req.(*CloseConnectionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_ConnectionSync_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ConnectionSyncRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).ConnectionSync(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/ConnectionSync",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).ConnectionSync(ctx, req.(*ConnectionSyncRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_DatabaseProperty_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DatabasePropertyRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).DatabaseProperty(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/DatabaseProperty",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).DatabaseProperty(ctx, req.(*DatabasePropertyRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Commit_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CommitRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Commit(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Commit",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Commit(ctx, req.(*CommitRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_Rollback_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RollbackRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).Rollback(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/Rollback",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).Rollback(ctx, req.(*RollbackRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_PrepareAndExecuteBatch_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PrepareAndExecuteBatchRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).PrepareAndExecuteBatch(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/PrepareAndExecuteBatch",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).PrepareAndExecuteBatch(ctx, req.(*PrepareAndExecuteBatchRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Avatica_ExecuteBatch_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ExecuteBatchRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(AvaticaServer).ExecuteBatch(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/avatica_grpc.Avatica/ExecuteBatch",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(AvaticaServer).ExecuteBatch(ctx, req.(*ExecuteBatchRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Avatica_ServiceDesc is the grpc.ServiceDesc for Avatica service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Avatica_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "avatica_grpc.Avatica",
	HandlerType: (*AvaticaServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Catalogs",
			Handler:    _Avatica_Catalogs_Handler,
		},
		{
			MethodName: "Schemas",
			Handler:    _Avatica_Schemas_Handler,
		},
		{
			MethodName: "Tables",
			Handler:    _Avatica_Tables_Handler,
		},
		{
			MethodName: "TableTypes",
			Handler:    _Avatica_TableTypes_Handler,
		},
		{
			MethodName: "TypeInfo",
			Handler:    _Avatica_TypeInfo_Handler,
		},
		{
			MethodName: "Columns",
			Handler:    _Avatica_Columns_Handler,
		},
		{
			MethodName: "Prepare",
			Handler:    _Avatica_Prepare_Handler,
		},
		{
			MethodName: "Execute",
			Handler:    _Avatica_Execute_Handler,
		},
		{
			MethodName: "PrepareAndExecute",
			Handler:    _Avatica_PrepareAndExecute_Handler,
		},
		{
			MethodName: "SyncResults",
			Handler:    _Avatica_SyncResults_Handler,
		},
		{
			MethodName: "Fetch",
			Handler:    _Avatica_Fetch_Handler,
		},
		{
			MethodName: "CreateStatement",
			Handler:    _Avatica_CreateStatement_Handler,
		},
		{
			MethodName: "CloseStatement",
			Handler:    _Avatica_CloseStatement_Handler,
		},
		{
			MethodName: "OpenConnection",
			Handler:    _Avatica_OpenConnection_Handler,
		},
		{
			MethodName: "CloseConnection",
			Handler:    _Avatica_CloseConnection_Handler,
		},
		{
			MethodName: "ConnectionSync",
			Handler:    _Avatica_ConnectionSync_Handler,
		},
		{
			MethodName: "DatabaseProperty",
			Handler:    _Avatica_DatabaseProperty_Handler,
		},
		{
			MethodName: "Commit",
			Handler:    _Avatica_Commit_Handler,
		},
		{
			MethodName: "Rollback",
			Handler:    _Avatica_Rollback_Handler,
		},
		{
			MethodName: "PrepareAndExecuteBatch",
			Handler:    _Avatica_PrepareAndExecuteBatch_Handler,
		},
		{
			MethodName: "ExecuteBatch",
			Handler:    _Avatica_ExecuteBatch_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "avatica.proto",
}
