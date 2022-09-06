/// Generated client implementations.
pub mod avatica_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// https://github.com/apache/calcite-avatica/blob/1f0f0c1c56b35c4524564a126f1db525437a130b/core/src/main/java/org/apache/calcite/avatica/remote/Service.java#L60-L80
    #[derive(Debug, Clone)]
    pub struct AvaticaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AvaticaClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AvaticaClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AvaticaClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AvaticaClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CatalogsRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Catalogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SchemasRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Schemas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tables(
            &mut self,
            request: impl tonic::IntoRequest<super::super::TablesRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Tables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn table_types(
            &mut self,
            request: impl tonic::IntoRequest<super::super::TableTypesRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/TableTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn type_info(
            &mut self,
            request: impl tonic::IntoRequest<super::super::TypeInfoRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/TypeInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn columns(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ColumnsRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Columns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::super::PrepareRequest>,
        ) -> Result<tonic::Response<super::super::PrepareResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Prepare",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Execute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn prepare_and_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::PrepareAndExecuteRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/PrepareAndExecute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn sync_results(
            &mut self,
            request: impl tonic::IntoRequest<super::super::SyncResultsRequest>,
        ) -> Result<tonic::Response<super::super::SyncResultsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/SyncResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn fetch(
            &mut self,
            request: impl tonic::IntoRequest<super::super::FetchRequest>,
        ) -> Result<tonic::Response<super::super::FetchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Fetch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CreateStatementRequest>,
        ) -> Result<
            tonic::Response<super::super::CreateStatementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/CreateStatement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CloseStatementRequest>,
        ) -> Result<
            tonic::Response<super::super::CloseStatementResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/CloseStatement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn open_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::super::OpenConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::OpenConnectionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/OpenConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CloseConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::CloseConnectionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/CloseConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn connection_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ConnectionSyncRequest>,
        ) -> Result<
            tonic::Response<super::super::ConnectionSyncResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/ConnectionSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn database_property(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DatabasePropertyRequest>,
        ) -> Result<
            tonic::Response<super::super::DatabasePropertyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/DatabaseProperty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CommitRequest>,
        ) -> Result<tonic::Response<super::super::CommitResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Commit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn rollback(
            &mut self,
            request: impl tonic::IntoRequest<super::super::RollbackRequest>,
        ) -> Result<tonic::Response<super::super::RollbackResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/Rollback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn prepare_and_execute_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::super::PrepareAndExecuteBatchRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/PrepareAndExecuteBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteBatchRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/avatica_grpc.Avatica/ExecuteBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod avatica_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AvaticaServer.
    #[async_trait]
    pub trait Avatica: Send + Sync + 'static {
        async fn catalogs(
            &self,
            request: tonic::Request<super::super::CatalogsRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn schemas(
            &self,
            request: tonic::Request<super::super::SchemasRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn tables(
            &self,
            request: tonic::Request<super::super::TablesRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn table_types(
            &self,
            request: tonic::Request<super::super::TableTypesRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn type_info(
            &self,
            request: tonic::Request<super::super::TypeInfoRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn columns(
            &self,
            request: tonic::Request<super::super::ColumnsRequest>,
        ) -> Result<tonic::Response<super::super::ResultSetResponse>, tonic::Status>;
        async fn prepare(
            &self,
            request: tonic::Request<super::super::PrepareRequest>,
        ) -> Result<tonic::Response<super::super::PrepareResponse>, tonic::Status>;
        async fn execute(
            &self,
            request: tonic::Request<super::super::ExecuteRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteResponse>, tonic::Status>;
        async fn prepare_and_execute(
            &self,
            request: tonic::Request<super::super::PrepareAndExecuteRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteResponse>, tonic::Status>;
        async fn sync_results(
            &self,
            request: tonic::Request<super::super::SyncResultsRequest>,
        ) -> Result<tonic::Response<super::super::SyncResultsResponse>, tonic::Status>;
        async fn fetch(
            &self,
            request: tonic::Request<super::super::FetchRequest>,
        ) -> Result<tonic::Response<super::super::FetchResponse>, tonic::Status>;
        async fn create_statement(
            &self,
            request: tonic::Request<super::super::CreateStatementRequest>,
        ) -> Result<
            tonic::Response<super::super::CreateStatementResponse>,
            tonic::Status,
        >;
        async fn close_statement(
            &self,
            request: tonic::Request<super::super::CloseStatementRequest>,
        ) -> Result<
            tonic::Response<super::super::CloseStatementResponse>,
            tonic::Status,
        >;
        async fn open_connection(
            &self,
            request: tonic::Request<super::super::OpenConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::OpenConnectionResponse>,
            tonic::Status,
        >;
        async fn close_connection(
            &self,
            request: tonic::Request<super::super::CloseConnectionRequest>,
        ) -> Result<
            tonic::Response<super::super::CloseConnectionResponse>,
            tonic::Status,
        >;
        async fn connection_sync(
            &self,
            request: tonic::Request<super::super::ConnectionSyncRequest>,
        ) -> Result<
            tonic::Response<super::super::ConnectionSyncResponse>,
            tonic::Status,
        >;
        async fn database_property(
            &self,
            request: tonic::Request<super::super::DatabasePropertyRequest>,
        ) -> Result<
            tonic::Response<super::super::DatabasePropertyResponse>,
            tonic::Status,
        >;
        async fn commit(
            &self,
            request: tonic::Request<super::super::CommitRequest>,
        ) -> Result<tonic::Response<super::super::CommitResponse>, tonic::Status>;
        async fn rollback(
            &self,
            request: tonic::Request<super::super::RollbackRequest>,
        ) -> Result<tonic::Response<super::super::RollbackResponse>, tonic::Status>;
        async fn prepare_and_execute_batch(
            &self,
            request: tonic::Request<super::super::PrepareAndExecuteBatchRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteBatchResponse>, tonic::Status>;
        async fn execute_batch(
            &self,
            request: tonic::Request<super::super::ExecuteBatchRequest>,
        ) -> Result<tonic::Response<super::super::ExecuteBatchResponse>, tonic::Status>;
    }
    /// https://github.com/apache/calcite-avatica/blob/1f0f0c1c56b35c4524564a126f1db525437a130b/core/src/main/java/org/apache/calcite/avatica/remote/Service.java#L60-L80
    #[derive(Debug)]
    pub struct AvaticaServer<T: Avatica> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Avatica> AvaticaServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AvaticaServer<T>
    where
        T: Avatica,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/avatica_grpc.Avatica/Catalogs" => {
                    #[allow(non_camel_case_types)]
                    struct CatalogsSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::CatalogsRequest>
                    for CatalogsSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::CatalogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).catalogs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CatalogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Schemas" => {
                    #[allow(non_camel_case_types)]
                    struct SchemasSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::SchemasRequest>
                    for SchemasSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::SchemasRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).schemas(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SchemasSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Tables" => {
                    #[allow(non_camel_case_types)]
                    struct TablesSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::TablesRequest>
                    for TablesSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::TablesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).tables(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TablesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/TableTypes" => {
                    #[allow(non_camel_case_types)]
                    struct TableTypesSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::TableTypesRequest>
                    for TableTypesSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::TableTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).table_types(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TableTypesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/TypeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct TypeInfoSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::TypeInfoRequest>
                    for TypeInfoSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::TypeInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).type_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TypeInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Columns" => {
                    #[allow(non_camel_case_types)]
                    struct ColumnsSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::ColumnsRequest>
                    for ColumnsSvc<T> {
                        type Response = super::super::ResultSetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::ColumnsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).columns(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ColumnsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Prepare" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::PrepareRequest>
                    for PrepareSvc<T> {
                        type Response = super::super::PrepareResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::PrepareRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).prepare(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Execute" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::ExecuteRequest>
                    for ExecuteSvc<T> {
                        type Response = super::super::ExecuteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::ExecuteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).execute(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/PrepareAndExecute" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareAndExecuteSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::PrepareAndExecuteRequest>
                    for PrepareAndExecuteSvc<T> {
                        type Response = super::super::ExecuteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::PrepareAndExecuteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_and_execute(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareAndExecuteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/SyncResults" => {
                    #[allow(non_camel_case_types)]
                    struct SyncResultsSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::SyncResultsRequest>
                    for SyncResultsSvc<T> {
                        type Response = super::super::SyncResultsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::SyncResultsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).sync_results(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SyncResultsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Fetch" => {
                    #[allow(non_camel_case_types)]
                    struct FetchSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::FetchRequest>
                    for FetchSvc<T> {
                        type Response = super::super::FetchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::FetchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fetch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/CreateStatement" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStatementSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::CreateStatementRequest>
                    for CreateStatementSvc<T> {
                        type Response = super::super::CreateStatementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::CreateStatementRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_statement(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateStatementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/CloseStatement" => {
                    #[allow(non_camel_case_types)]
                    struct CloseStatementSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::CloseStatementRequest>
                    for CloseStatementSvc<T> {
                        type Response = super::super::CloseStatementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::CloseStatementRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).close_statement(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseStatementSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/OpenConnection" => {
                    #[allow(non_camel_case_types)]
                    struct OpenConnectionSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::OpenConnectionRequest>
                    for OpenConnectionSvc<T> {
                        type Response = super::super::OpenConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::OpenConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).open_connection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/CloseConnection" => {
                    #[allow(non_camel_case_types)]
                    struct CloseConnectionSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::CloseConnectionRequest>
                    for CloseConnectionSvc<T> {
                        type Response = super::super::CloseConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::CloseConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).close_connection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/ConnectionSync" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionSyncSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::ConnectionSyncRequest>
                    for ConnectionSyncSvc<T> {
                        type Response = super::super::ConnectionSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::ConnectionSyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).connection_sync(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionSyncSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/DatabaseProperty" => {
                    #[allow(non_camel_case_types)]
                    struct DatabasePropertySvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::DatabasePropertyRequest>
                    for DatabasePropertySvc<T> {
                        type Response = super::super::DatabasePropertyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::DatabasePropertyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).database_property(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DatabasePropertySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Commit" => {
                    #[allow(non_camel_case_types)]
                    struct CommitSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::CommitRequest>
                    for CommitSvc<T> {
                        type Response = super::super::CommitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::CommitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).commit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CommitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/Rollback" => {
                    #[allow(non_camel_case_types)]
                    struct RollbackSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::RollbackRequest>
                    for RollbackSvc<T> {
                        type Response = super::super::RollbackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::RollbackRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).rollback(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RollbackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/PrepareAndExecuteBatch" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareAndExecuteBatchSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<
                        super::super::PrepareAndExecuteBatchRequest,
                    > for PrepareAndExecuteBatchSvc<T> {
                        type Response = super::super::ExecuteBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::PrepareAndExecuteBatchRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_and_execute_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareAndExecuteBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/avatica_grpc.Avatica/ExecuteBatch" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteBatchSvc<T: Avatica>(pub Arc<T>);
                    impl<
                        T: Avatica,
                    > tonic::server::UnaryService<super::super::ExecuteBatchRequest>
                    for ExecuteBatchSvc<T> {
                        type Response = super::super::ExecuteBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::ExecuteBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Avatica> Clone for AvaticaServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Avatica> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Avatica> tonic::server::NamedService for AvaticaServer<T> {
        const NAME: &'static str = "avatica_grpc.Avatica";
    }
}
