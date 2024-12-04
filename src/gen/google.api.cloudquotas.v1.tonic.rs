// @generated
/// Generated client implementations.
pub mod cloud_quotas_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CloudQuotasClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudQuotasClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CloudQuotasClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> CloudQuotasClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            CloudQuotasClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn list_quota_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaInfosResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "ListQuotaInfos",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_quota_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQuotaInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "GetQuotaInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_quota_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaPreferencesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "ListQuotaPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "GetQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/CreateQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "CreateQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/UpdateQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "UpdateQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cloud_quotas_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CloudQuotasServer.
    #[async_trait]
    pub trait CloudQuotas: std::marker::Send + std::marker::Sync + 'static {
        async fn list_quota_infos(
            &self,
            request: tonic::Request<super::ListQuotaInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaInfosResponse>,
            tonic::Status,
        >;
        async fn get_quota_info(
            &self,
            request: tonic::Request<super::GetQuotaInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaInfo>, tonic::Status>;
        async fn list_quota_preferences(
            &self,
            request: tonic::Request<super::ListQuotaPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaPreferencesResponse>,
            tonic::Status,
        >;
        async fn get_quota_preference(
            &self,
            request: tonic::Request<super::GetQuotaPreferenceRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaPreference>, tonic::Status>;
        async fn create_quota_preference(
            &self,
            request: tonic::Request<super::CreateQuotaPreferenceRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaPreference>, tonic::Status>;
        async fn update_quota_preference(
            &self,
            request: tonic::Request<super::UpdateQuotaPreferenceRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaPreference>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CloudQuotasServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> CloudQuotasServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CloudQuotasServer<T>
    where
        T: CloudQuotas,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaInfos" => {
                    #[allow(non_camel_case_types)]
                    struct ListQuotaInfosSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::ListQuotaInfosRequest>
                    for ListQuotaInfosSvc<T> {
                        type Response = super::ListQuotaInfosResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQuotaInfosRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::list_quota_infos(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListQuotaInfosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetQuotaInfoSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::GetQuotaInfoRequest>
                    for GetQuotaInfoSvc<T> {
                        type Response = super::QuotaInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQuotaInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::get_quota_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetQuotaInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaPreferences" => {
                    #[allow(non_camel_case_types)]
                    struct ListQuotaPreferencesSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::ListQuotaPreferencesRequest>
                    for ListQuotaPreferencesSvc<T> {
                        type Response = super::ListQuotaPreferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQuotaPreferencesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::list_quota_preferences(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListQuotaPreferencesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaPreference" => {
                    #[allow(non_camel_case_types)]
                    struct GetQuotaPreferenceSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::GetQuotaPreferenceRequest>
                    for GetQuotaPreferenceSvc<T> {
                        type Response = super::QuotaPreference;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetQuotaPreferenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::get_quota_preference(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetQuotaPreferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.api.cloudquotas.v1.CloudQuotas/CreateQuotaPreference" => {
                    #[allow(non_camel_case_types)]
                    struct CreateQuotaPreferenceSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::CreateQuotaPreferenceRequest>
                    for CreateQuotaPreferenceSvc<T> {
                        type Response = super::QuotaPreference;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateQuotaPreferenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::create_quota_preference(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateQuotaPreferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.api.cloudquotas.v1.CloudQuotas/UpdateQuotaPreference" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateQuotaPreferenceSvc<T: CloudQuotas>(pub Arc<T>);
                    impl<
                        T: CloudQuotas,
                    > tonic::server::UnaryService<super::UpdateQuotaPreferenceRequest>
                    for UpdateQuotaPreferenceSvc<T> {
                        type Response = super::QuotaPreference;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateQuotaPreferenceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CloudQuotas>::update_quota_preference(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateQuotaPreferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for CloudQuotasServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.api.cloudquotas.v1.CloudQuotas";
    impl<T> tonic::server::NamedService for CloudQuotasServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
