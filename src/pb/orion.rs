#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vocabulary {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / 单词
    #[prost(string, tag = "2")]
    pub word: ::prost::alloc::string::String,
    /// / 音标
    #[prost(string, tag = "3")]
    pub soundmark: ::prost::alloc::string::String,
    /// / 词根
    #[prost(string, tag = "4")]
    pub roots: ::prost::alloc::string::String,
    /// / 释义
    #[prost(string, tag = "5")]
    pub paraphrase: ::prost::alloc::string::String,
    /// / 词组
    #[prost(string, tag = "6")]
    pub collocations: ::prost::alloc::string::String,
    /// / 同义词
    #[prost(string, tag = "7")]
    pub synonyms: ::prost::alloc::string::String,
    /// / 例句
    #[prost(string, tag = "8")]
    pub examples: ::prost::alloc::string::String,
    /// / 创建时间
    #[prost(message, optional, tag = "9")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / 更新时间
    #[prost(message, optional, tag = "10")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]
#[builder(setter(into), default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / 单词
    #[prost(string, optional, tag = "2")]
    pub word: ::core::option::Option<::prost::alloc::string::String>,
    /// / 音标
    #[prost(string, optional, tag = "3")]
    pub soundmark: ::core::option::Option<::prost::alloc::string::String>,
    /// / 词根
    #[prost(string, optional, tag = "4")]
    pub roots: ::core::option::Option<::prost::alloc::string::String>,
    /// / 释义
    #[prost(string, optional, tag = "5")]
    pub paraphrase: ::core::option::Option<::prost::alloc::string::String>,
    /// / 词组
    #[prost(string, optional, tag = "6")]
    pub collocations: ::core::option::Option<::prost::alloc::string::String>,
    /// / 同义词
    #[prost(string, optional, tag = "7")]
    pub synonyms: ::core::option::Option<::prost::alloc::string::String>,
    /// / 例句
    #[prost(string, optional, tag = "8")]
    pub examples: ::core::option::Option<::prost::alloc::string::String>,
    /// / offset
    #[prost(int64, optional, tag = "9")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "10")]
    pub limit: ::core::option::Option<i64>,
}
/// add a new vocabulary
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddVocabularyRequest {
    #[prost(message, optional, tag = "1")]
    pub vocabulary: ::core::option::Option<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyResponse {
    #[prost(message, optional, tag = "1")]
    pub vocabulary: ::core::option::Option<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<VocabularyQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyResponse {
    #[prost(message, repeated, tag = "1")]
    pub vocabulary: ::prost::alloc::vec::Vec<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyRandomRequest {
    #[prost(int64, tag = "1")]
    pub limit: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Story {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / words
    #[prost(string, repeated, tag = "2")]
    pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// / content
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// / read count
    #[prost(int64, tag = "4")]
    pub read_count: i64,
    /// / created at
    #[prost(message, optional, tag = "5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / updated at
    #[prost(message, optional, tag = "6")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]
#[builder(setter(into), default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / words
    #[prost(string, optional, tag = "2")]
    pub words: ::core::option::Option<::prost::alloc::string::String>,
    /// / content
    #[prost(string, optional, tag = "3")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    /// / read count
    #[prost(int64, optional, tag = "4")]
    pub read_count: ::core::option::Option<i64>,
    /// / offset
    #[prost(int64, optional, tag = "5")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "6")]
    pub limit: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStoryRequest {
    #[prost(message, optional, tag = "1")]
    pub story: ::core::option::Option<Story>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryResponse {
    #[prost(message, optional, tag = "1")]
    pub story: ::core::option::Option<Story>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub story: ::prost::alloc::vec::Vec<Story>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LearnWord {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / word
    #[prost(string, tag = "2")]
    pub word: ::prost::alloc::string::String,
    /// / vocabulary id
    #[prost(int64, tag = "3")]
    pub vocabulary_id: i64,
    /// / word list id
    #[prost(int64, tag = "4")]
    pub word_list_id: i64,
    /// / learn count
    #[prost(int64, tag = "5")]
    pub learn_count: i64,
    /// / learn status
    #[prost(enumeration = "LearnStatus", tag = "6")]
    pub learn_status: i32,
    /// / last learned at
    #[prost(message, optional, tag = "7")]
    pub last_learned_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / next learn at
    #[prost(message, optional, tag = "8")]
    pub next_learn_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / created at
    #[prost(message, optional, tag = "9")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / updated at
    #[prost(message, optional, tag = "10")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]
#[builder(setter(into), default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LearnWordQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / word
    #[prost(string, optional, tag = "2")]
    pub word: ::core::option::Option<::prost::alloc::string::String>,
    /// / vocabulary id
    #[prost(int64, optional, tag = "3")]
    pub vocabulary_id: ::core::option::Option<i64>,
    /// / word list id
    #[prost(int64, optional, tag = "4")]
    pub word_list_id: ::core::option::Option<i64>,
    /// / learn count
    #[prost(int64, optional, tag = "5")]
    pub learn_count: ::core::option::Option<i64>,
    /// / learn status
    #[prost(enumeration = "LearnStatus", optional, tag = "6")]
    pub learn_status: ::core::option::Option<i32>,
    /// / last learned at
    #[prost(message, optional, tag = "7")]
    pub last_learned_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / next learn at
    #[prost(message, optional, tag = "8")]
    pub next_learn_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / offset
    #[prost(int64, optional, tag = "9")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "10")]
    pub limit: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLearnWordRequest {
    #[prost(message, optional, tag = "1")]
    pub word: ::core::option::Option<LearnWord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LearnWordResponse {
    #[prost(message, optional, tag = "1")]
    pub word: ::core::option::Option<LearnWord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLearnWordRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<LearnWordQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLearnWordResponse {
    #[prost(message, repeated, tag = "1")]
    pub word: ::prost::alloc::vec::Vec<LearnWord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordList {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / name
    #[prost(string, tag = "2")]
    pub word: ::prost::alloc::string::String,
    /// / paraphrase
    #[prost(string, tag = "3")]
    pub paraphrase: ::prost::alloc::string::String,
    /// / word classification
    #[prost(enumeration = "WordClassification", tag = "4")]
    pub classification: i32,
    /// / created at
    #[prost(message, optional, tag = "5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / updated at
    #[prost(message, optional, tag = "6")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]
#[builder(setter(into), default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordListQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / name
    #[prost(string, optional, tag = "2")]
    pub word: ::core::option::Option<::prost::alloc::string::String>,
    /// / paraphrase
    #[prost(string, optional, tag = "3")]
    pub paraphrase: ::core::option::Option<::prost::alloc::string::String>,
    /// / word classification
    #[prost(enumeration = "WordClassification", optional, tag = "4")]
    pub classification: ::core::option::Option<i32>,
    /// / offset
    #[prost(int64, optional, tag = "5")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "6")]
    pub limit: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWordListRequest {
    #[prost(message, optional, tag = "1")]
    pub word: ::core::option::Option<WordList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordListResponse {
    #[prost(message, optional, tag = "1")]
    pub word: ::core::option::Option<WordList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWordListRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<WordListQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWordListResponse {
    #[prost(message, repeated, tag = "1")]
    pub word: ::prost::alloc::vec::Vec<WordList>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LearnStatus {
    New = 0,
    Easy = 1,
    Difficult = 2,
    Learned = 3,
}
impl LearnStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LearnStatus::New => "LEARN_STATUS_NEW",
            LearnStatus::Easy => "LEARN_STATUS_EASY",
            LearnStatus::Difficult => "LEARN_STATUS_DIFFICULT",
            LearnStatus::Learned => "LEARN_STATUS_LEARNED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEARN_STATUS_NEW" => Some(Self::New),
            "LEARN_STATUS_EASY" => Some(Self::Easy),
            "LEARN_STATUS_DIFFICULT" => Some(Self::Difficult),
            "LEARN_STATUS_LEARNED" => Some(Self::Learned),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WordClassification {
    Cet4 = 0,
    Cet6 = 1,
    Junior = 2,
    Senior = 3,
    Graduate = 4,
    Toefl = 5,
    Sat = 6,
    Unkown = 7,
}
impl WordClassification {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WordClassification::Cet4 => "CET4",
            WordClassification::Cet6 => "CET6",
            WordClassification::Junior => "JUNIOR",
            WordClassification::Senior => "SENIOR",
            WordClassification::Graduate => "GRADUATE",
            WordClassification::Toefl => "TOEFL",
            WordClassification::Sat => "SAT",
            WordClassification::Unkown => "UNKOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CET4" => Some(Self::Cet4),
            "CET6" => Some(Self::Cet6),
            "JUNIOR" => Some(Self::Junior),
            "SENIOR" => Some(Self::Senior),
            "GRADUATE" => Some(Self::Graduate),
            "TOEFL" => Some(Self::Toefl),
            "SAT" => Some(Self::Sat),
            "UNKOWN" => Some(Self::Unkown),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod vocabulary_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// The Vocabulary service definition.
    #[derive(Debug, Clone)]
    pub struct VocabularyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VocabularyServiceClient<tonic::transport::Channel> {
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
    impl<T> VocabularyServiceClient<T>
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
        ) -> VocabularyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            VocabularyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// add a new vocabulary
        pub async fn add_vocabulary(
            &mut self,
            request: impl tonic::IntoRequest<super::AddVocabularyRequest>,
        ) -> Result<tonic::Response<super::VocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/orion.VocabularyService/AddVocabulary");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query vocabulary
        pub async fn query_vocabulary(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVocabularyRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/orion.VocabularyService/QueryVocabulary");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query random vocabulary
        pub async fn query_vocabulary_random(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVocabularyRandomRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orion.VocabularyService/QueryVocabularyRandom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod story_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StoryServiceClient<tonic::transport::Channel> {
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
    impl<T> StoryServiceClient<T>
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
        ) -> StoryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StoryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// add a new story
        pub async fn add_story(
            &mut self,
            request: impl tonic::IntoRequest<super::AddStoryRequest>,
        ) -> Result<tonic::Response<super::StoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/orion.StoryService/AddStory");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod learn_word_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LearnWordServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LearnWordServiceClient<tonic::transport::Channel> {
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
    impl<T> LearnWordServiceClient<T>
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
        ) -> LearnWordServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            LearnWordServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// add a new learn word
        pub async fn add_learn_word(
            &mut self,
            request: impl tonic::IntoRequest<super::AddLearnWordRequest>,
        ) -> Result<tonic::Response<super::LearnWordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/orion.LearnWordService/AddLearnWord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query learn word
        pub async fn query_learn_word(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLearnWordRequest>,
        ) -> Result<tonic::Response<super::QueryLearnWordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/orion.LearnWordService/QueryLearnWord");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod word_list_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WordListServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WordListServiceClient<tonic::transport::Channel> {
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
    impl<T> WordListServiceClient<T>
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
        ) -> WordListServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            WordListServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// add a new word list
        pub async fn add_word_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AddWordListRequest>,
        ) -> Result<tonic::Response<super::WordListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/orion.WordListService/AddWordList");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query word list
        pub async fn query_word_list(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWordListRequest>,
        ) -> Result<tonic::Response<super::QueryWordListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/orion.WordListService/QueryWordList");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vocabulary_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VocabularyServiceServer.
    #[async_trait]
    pub trait VocabularyService: Send + Sync + 'static {
        /// add a new vocabulary
        async fn add_vocabulary(
            &self,
            request: tonic::Request<super::AddVocabularyRequest>,
        ) -> Result<tonic::Response<super::VocabularyResponse>, tonic::Status>;
        /// query vocabulary
        async fn query_vocabulary(
            &self,
            request: tonic::Request<super::QueryVocabularyRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status>;
        /// query random vocabulary
        async fn query_vocabulary_random(
            &self,
            request: tonic::Request<super::QueryVocabularyRandomRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status>;
    }
    /// The Vocabulary service definition.
    #[derive(Debug)]
    pub struct VocabularyServiceServer<T: VocabularyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VocabularyService> VocabularyServiceServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VocabularyServiceServer<T>
    where
        T: VocabularyService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.VocabularyService/AddVocabulary" => {
                    #[allow(non_camel_case_types)]
                    struct AddVocabularySvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::AddVocabularyRequest>
                        for AddVocabularySvc<T>
                    {
                        type Response = super::VocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddVocabularyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_vocabulary(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddVocabularySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.VocabularyService/QueryVocabulary" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVocabularySvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::QueryVocabularyRequest>
                        for QueryVocabularySvc<T>
                    {
                        type Response = super::QueryVocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVocabularyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_vocabulary(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVocabularySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.VocabularyService/QueryVocabularyRandom" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVocabularyRandomSvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::QueryVocabularyRandomRequest>
                        for QueryVocabularyRandomSvc<T>
                    {
                        type Response = super::QueryVocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVocabularyRandomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_vocabulary_random(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVocabularyRandomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: VocabularyService> Clone for VocabularyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VocabularyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VocabularyService> tonic::server::NamedService for VocabularyServiceServer<T> {
        const NAME: &'static str = "orion.VocabularyService";
    }
}
/// Generated server implementations.
pub mod story_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StoryServiceServer.
    #[async_trait]
    pub trait StoryService: Send + Sync + 'static {
        /// add a new story
        async fn add_story(
            &self,
            request: tonic::Request<super::AddStoryRequest>,
        ) -> Result<tonic::Response<super::StoryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StoryServiceServer<T: StoryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StoryService> StoryServiceServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StoryServiceServer<T>
    where
        T: StoryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.StoryService/AddStory" => {
                    #[allow(non_camel_case_types)]
                    struct AddStorySvc<T: StoryService>(pub Arc<T>);
                    impl<T: StoryService> tonic::server::UnaryService<super::AddStoryRequest> for AddStorySvc<T> {
                        type Response = super::StoryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddStoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_story(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddStorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: StoryService> Clone for StoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StoryService> tonic::server::NamedService for StoryServiceServer<T> {
        const NAME: &'static str = "orion.StoryService";
    }
}
/// Generated server implementations.
pub mod learn_word_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LearnWordServiceServer.
    #[async_trait]
    pub trait LearnWordService: Send + Sync + 'static {
        /// add a new learn word
        async fn add_learn_word(
            &self,
            request: tonic::Request<super::AddLearnWordRequest>,
        ) -> Result<tonic::Response<super::LearnWordResponse>, tonic::Status>;
        /// query learn word
        async fn query_learn_word(
            &self,
            request: tonic::Request<super::QueryLearnWordRequest>,
        ) -> Result<tonic::Response<super::QueryLearnWordResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LearnWordServiceServer<T: LearnWordService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LearnWordService> LearnWordServiceServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LearnWordServiceServer<T>
    where
        T: LearnWordService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.LearnWordService/AddLearnWord" => {
                    #[allow(non_camel_case_types)]
                    struct AddLearnWordSvc<T: LearnWordService>(pub Arc<T>);
                    impl<T: LearnWordService>
                        tonic::server::UnaryService<super::AddLearnWordRequest>
                        for AddLearnWordSvc<T>
                    {
                        type Response = super::LearnWordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddLearnWordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_learn_word(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLearnWordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.LearnWordService/QueryLearnWord" => {
                    #[allow(non_camel_case_types)]
                    struct QueryLearnWordSvc<T: LearnWordService>(pub Arc<T>);
                    impl<T: LearnWordService>
                        tonic::server::UnaryService<super::QueryLearnWordRequest>
                        for QueryLearnWordSvc<T>
                    {
                        type Response = super::QueryLearnWordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLearnWordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_learn_word(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryLearnWordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: LearnWordService> Clone for LearnWordServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: LearnWordService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LearnWordService> tonic::server::NamedService for LearnWordServiceServer<T> {
        const NAME: &'static str = "orion.LearnWordService";
    }
}
/// Generated server implementations.
pub mod word_list_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WordListServiceServer.
    #[async_trait]
    pub trait WordListService: Send + Sync + 'static {
        /// add a new word list
        async fn add_word_list(
            &self,
            request: tonic::Request<super::AddWordListRequest>,
        ) -> Result<tonic::Response<super::WordListResponse>, tonic::Status>;
        /// query word list
        async fn query_word_list(
            &self,
            request: tonic::Request<super::QueryWordListRequest>,
        ) -> Result<tonic::Response<super::QueryWordListResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct WordListServiceServer<T: WordListService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WordListService> WordListServiceServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WordListServiceServer<T>
    where
        T: WordListService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.WordListService/AddWordList" => {
                    #[allow(non_camel_case_types)]
                    struct AddWordListSvc<T: WordListService>(pub Arc<T>);
                    impl<T: WordListService> tonic::server::UnaryService<super::AddWordListRequest>
                        for AddWordListSvc<T>
                    {
                        type Response = super::WordListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddWordListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_word_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddWordListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.WordListService/QueryWordList" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWordListSvc<T: WordListService>(pub Arc<T>);
                    impl<T: WordListService>
                        tonic::server::UnaryService<super::QueryWordListRequest>
                        for QueryWordListSvc<T>
                    {
                        type Response = super::QueryWordListResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWordListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_word_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryWordListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: WordListService> Clone for WordListServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WordListService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WordListService> tonic::server::NamedService for WordListServiceServer<T> {
        const NAME: &'static str = "orion.WordListService";
    }
}
