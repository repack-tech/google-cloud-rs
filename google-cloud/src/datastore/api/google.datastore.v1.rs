/// A partition ID identifies a grouping of entities. The grouping is always
/// by project and namespace, however the namespace ID may be empty.
///
/// A partition ID contains several dimensions:
/// project ID and namespace ID.
///
/// Partition dimensions:
///
/// - May be `""`.
/// - Must be valid UTF-8 bytes.
/// - Must have values that match regex `[A-Za-z\d\.\-_]{1,100}`
/// If the value of any dimension matches regex `__.*__`, the partition is
/// reserved/read-only.
/// A reserved/read-only partition ID is forbidden in certain documented
/// contexts.
///
/// Foreign partition IDs (in which the project ID does
/// not match the context project ID ) are discouraged.
/// Reads and writes of foreign partition IDs may fail if the project is not in
/// an active state.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct PartitionId {
    /// The ID of the project to which the entities belong.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// If not empty, the ID of the namespace to which the entities belong.
    #[prost(string, tag = "4")]
    pub namespace_id: ::prost::alloc::string::String,
}
/// A unique identifier for an entity.
/// If a key's partition ID or any of its path kinds or names are
/// reserved/read-only, the key is reserved/read-only.
/// A reserved/read-only key is forbidden in certain documented contexts.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct Key {
    /// Entities are partitioned into subsets, currently identified by a project
    /// ID and namespace ID.
    /// Queries are scoped to a single partition.
    #[prost(message, optional, tag = "1")]
    pub partition_id: ::core::option::Option<PartitionId>,
    /// The entity path.
    /// An entity path consists of one or more elements composed of a kind and a
    /// string or numerical identifier, which identify entities. The first
    /// element identifies a _root entity_, the second element identifies
    /// a _child_ of the root entity, the third element identifies a child of the
    /// second entity, and so forth. The entities identified by all prefixes of
    /// the path are called the element's _ancestors_.
    ///
    /// An entity path is always fully complete: *all* of the entity's ancestors
    /// are required to be in the path along with the entity identifier itself.
    /// The only exception is that in some documented cases, the identifier in the
    /// last path element (for the entity) itself may be omitted. For example,
    /// the last path element of the key of `Mutation.insert` may have no
    /// identifier.
    ///
    /// A path can never be empty, and a path can have at most 100 elements.
    #[prost(message, repeated, tag = "2")]
    pub path: ::prost::alloc::vec::Vec<key::PathElement>,
}
/// Nested message and enum types in `Key`.
pub mod key {
    /// A (kind, ID/name) pair used to construct a key path.
    ///
    /// If either name or ID is set, the element is complete.
    /// If neither is set, the element is incomplete.
    #[derive(Clone, PartialEq, Eq, ::prost::Message)]
    pub struct PathElement {
        /// The kind of the entity.
        /// A kind matching regex `__.*__` is reserved/read-only.
        /// A kind must not contain more than 1500 bytes when UTF-8 encoded.
        /// Cannot be `""`.
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        /// The type of ID.
        #[prost(oneof = "path_element::IdType", tags = "2, 3")]
        pub id_type: ::core::option::Option<path_element::IdType>,
    }
    /// Nested message and enum types in `PathElement`.
    pub mod path_element {
        /// The type of ID.
        #[derive(Clone, PartialEq, Eq, ::prost::Oneof)]
        pub enum IdType {
            /// The auto-allocated ID of the entity.
            /// Never equal to zero. Values less than zero are discouraged and may not
            /// be supported in the future.
            #[prost(int64, tag = "2")]
            Id(i64),
            /// The name of the entity.
            /// A name matching regex `__.*__` is reserved/read-only.
            /// A name must not be more than 1500 bytes when UTF-8 encoded.
            /// Cannot be `""`.
            #[prost(string, tag = "3")]
            Name(::prost::alloc::string::String),
        }
    }
}
/// An array value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayValue {
    /// Values in the array.
    /// The order of this array may not be preserved if it contains a mix of
    /// indexed and unindexed values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// A message that can hold any of the supported value types and associated
/// metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The `meaning` field should only be populated for backwards compatibility.
    #[prost(int32, tag = "14")]
    pub meaning: i32,
    /// If the value should be excluded from all indexes including those defined
    /// explicitly.
    #[prost(bool, tag = "19")]
    pub exclude_from_indexes: bool,
    /// Must have a value set.
    #[prost(
        oneof = "value::ValueType",
        tags = "11, 1, 2, 3, 10, 5, 17, 18, 8, 6, 9"
    )]
    pub value_type: ::core::option::Option<value::ValueType>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// Must have a value set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueType {
        /// A null value.
        #[prost(enumeration = "::prost_types::NullValue", tag = "11")]
        NullValue(i32),
        /// A boolean value.
        #[prost(bool, tag = "1")]
        BooleanValue(bool),
        /// An integer value.
        #[prost(int64, tag = "2")]
        IntegerValue(i64),
        /// A double value.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// A timestamp value.
        /// When stored in the Datastore, precise only to microseconds;
        /// any additional precision is rounded down.
        #[prost(message, tag = "10")]
        TimestampValue(::prost_types::Timestamp),
        /// A key value.
        #[prost(message, tag = "5")]
        KeyValue(super::Key),
        /// A UTF-8 encoded string value.
        /// When `exclude_from_indexes` is false (it is indexed) , may have at most
        /// 1500 bytes. Otherwise, may be set to at least 1,000,000 bytes.
        #[prost(string, tag = "17")]
        StringValue(::prost::alloc::string::String),
        /// A blob value.
        /// May have at most 1,000,000 bytes.
        /// When `exclude_from_indexes` is false, may have at most 1500 bytes.
        /// In JSON requests, must be base64-encoded.
        #[prost(bytes, tag = "18")]
        BlobValue(::prost::alloc::vec::Vec<u8>),
        /// A geo point value representing a point on the surface of Earth.
        #[prost(message, tag = "8")]
        GeoPointValue(super::super::super::r#type::LatLng),
        /// An entity value.
        ///
        /// - May have no key.
        /// - May have a key with an incomplete key path.
        /// - May have a reserved/read-only key.
        #[prost(message, tag = "6")]
        EntityValue(super::Entity),
        /// An array value.
        /// Cannot contain another array value.
        /// A `Value` instance that sets field `array_value` must not set fields
        /// `meaning` or `exclude_from_indexes`.
        #[prost(message, tag = "9")]
        ArrayValue(super::ArrayValue),
    }
}
/// A Datastore data object.
///
/// An entity is limited to 1 megabyte when stored. That _roughly_
/// corresponds to a limit of 1 megabyte for the serialized form of this
/// message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// The entity's key.
    ///
    /// An entity must have a key, unless otherwise documented (for example,
    /// an entity in `Value.entity_value` may have no key).
    /// An entity's kind is its key path's last element's kind,
    /// or null if it has no key.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<Key>,
    /// The entity's properties.
    /// The map's keys are property names.
    /// A property name matching regex `__.*__` is reserved.
    /// A reserved property name is forbidden in certain documented contexts.
    /// The name must not contain more than 500 characters.
    /// The name cannot be `""`.
    #[prost(map = "string, message", tag = "3")]
    pub properties: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
/// The result of fetching an entity from Datastore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityResult {
    /// The resulting entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::core::option::Option<Entity>,
    /// The version of the entity, a strictly positive number that monotonically
    /// increases with changes to the entity.
    ///
    /// This field is set for [`FULL`][google.datastore.v1.EntityResult.ResultType.FULL] entity
    /// results.
    ///
    /// For [missing][google.datastore.v1.LookupResponse.missing] entities in `LookupResponse`, this
    /// is the version of the snapshot that was used to look up the entity, and it
    /// is always set except for eventually consistent reads.
    #[prost(int64, tag = "4")]
    pub version: i64,
    /// The time at which the entity was last changed.
    /// This field is set for [`FULL`][google.datastore.v1.EntityResult.ResultType.FULL] entity
    /// results.
    /// If this entity is missing, this field will not be set.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A cursor that points to the position after the result entity.
    /// Set only when the `EntityResult` is part of a `QueryResultBatch` message.
    #[prost(bytes = "vec", tag = "3")]
    pub cursor: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `EntityResult`.
pub mod entity_result {
    /// Specifies what data the 'entity' field contains.
    /// A `ResultType` is either implied (for example, in `LookupResponse.missing`
    /// from `datastore.proto`, it is always `KEY_ONLY`) or specified by context
    /// (for example, in message `QueryResultBatch`, field `entity_result_type`
    /// specifies a `ResultType` for all the values in field `entity_results`).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResultType {
        /// Unspecified. This value is never used.
        Unspecified = 0,
        /// The key and properties.
        Full = 1,
        /// A projected subset of properties. The entity may have no key.
        Projection = 2,
        /// Only the key.
        KeyOnly = 3,
    }
}
/// A query for entities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    /// The projection to return. Defaults to returning all properties.
    #[prost(message, repeated, tag = "2")]
    pub projection: ::prost::alloc::vec::Vec<Projection>,
    /// The kinds to query (if empty, returns entities of all kinds).
    /// Currently at most 1 kind may be specified.
    #[prost(message, repeated, tag = "3")]
    pub kind: ::prost::alloc::vec::Vec<KindExpression>,
    /// The filter to apply.
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<Filter>,
    /// The order to apply to the query results (if empty, order is unspecified).
    #[prost(message, repeated, tag = "5")]
    pub order: ::prost::alloc::vec::Vec<PropertyOrder>,
    /// The properties to make distinct. The query results will contain the first
    /// result for each distinct combination of values for the given properties
    /// (if empty, all results are returned).
    #[prost(message, repeated, tag = "6")]
    pub distinct_on: ::prost::alloc::vec::Vec<PropertyReference>,
    /// A starting point for the query results. Query cursors are
    /// returned in query result batches and
    /// [can only be used to continue the same
    /// query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).
    #[prost(bytes = "vec", tag = "7")]
    pub start_cursor: ::prost::alloc::vec::Vec<u8>,
    /// An ending point for the query results. Query cursors are
    /// returned in query result batches and
    /// [can only be used to limit the same
    /// query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets).
    #[prost(bytes = "vec", tag = "8")]
    pub end_cursor: ::prost::alloc::vec::Vec<u8>,
    /// The number of results to skip. Applies before limit, but after all other
    /// constraints. Optional. Must be >= 0 if specified.
    #[prost(int32, tag = "10")]
    pub offset: i32,
    /// The maximum number of results to return. Applies after all other
    /// constraints. Optional.
    /// Unspecified is interpreted as no limit.
    /// Must be >= 0 if specified.
    #[prost(message, optional, tag = "12")]
    pub limit: ::core::option::Option<i32>,
}
/// A representation of a kind.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct KindExpression {
    /// The name of the kind.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A reference to a property relative to the kind expressions.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct PropertyReference {
    /// The name of the property.
    /// If name includes "."s, it may be interpreted as a property name path.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// A representation of a property in a projection.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct Projection {
    /// The property to project.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<PropertyReference>,
}
/// The desired order for a specific property.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct PropertyOrder {
    /// The property to order by.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<PropertyReference>,
    /// The direction to order by. Defaults to `ASCENDING`.
    #[prost(enumeration = "property_order::Direction", tag = "2")]
    pub direction: i32,
}
/// Nested message and enum types in `PropertyOrder`.
pub mod property_order {
    /// The sort direction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// Unspecified. This value must not be used.
        Unspecified = 0,
        /// Ascending.
        Ascending = 1,
        /// Descending.
        Descending = 2,
    }
}
/// A holder for any type of filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The type of filter.
    #[prost(oneof = "filter::FilterType", tags = "1, 2")]
    pub filter_type: ::core::option::Option<filter::FilterType>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    /// The type of filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FilterType {
        /// A composite filter.
        #[prost(message, tag = "1")]
        CompositeFilter(super::CompositeFilter),
        /// A filter on a property.
        #[prost(message, tag = "2")]
        PropertyFilter(super::PropertyFilter),
    }
}
/// A filter that merges multiple other filters using the given operator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositeFilter {
    /// The operator for combining multiple filters.
    #[prost(enumeration = "composite_filter::Operator", tag = "1")]
    pub op: i32,
    /// The list of filters to combine.
    ///
    /// Requires:
    ///
    /// * At least one filter is present.
    #[prost(message, repeated, tag = "2")]
    pub filters: ::prost::alloc::vec::Vec<Filter>,
}
/// Nested message and enum types in `CompositeFilter`.
pub mod composite_filter {
    /// A composite filter operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operator {
        /// Unspecified. This value must not be used.
        Unspecified = 0,
        /// The results are required to satisfy each of the combined filters.
        And = 1,
    }
}
/// A filter on a specific property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyFilter {
    /// The property to filter by.
    #[prost(message, optional, tag = "1")]
    pub property: ::core::option::Option<PropertyReference>,
    /// The operator to filter by.
    #[prost(enumeration = "property_filter::Operator", tag = "2")]
    pub op: i32,
    /// The value to compare the property to.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<Value>,
}
/// Nested message and enum types in `PropertyFilter`.
pub mod property_filter {
    /// A property filter operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operator {
        /// Unspecified. This value must not be used.
        Unspecified = 0,
        /// The given `property` is less than the given `value`.
        ///
        /// Requires:
        ///
        /// * That `property` comes first in `order_by`.
        LessThan = 1,
        /// The given `property` is less than or equal to the given `value`.
        ///
        /// Requires:
        ///
        /// * That `property` comes first in `order_by`.
        LessThanOrEqual = 2,
        /// The given `property` is greater than the given `value`.
        ///
        /// Requires:
        ///
        /// * That `property` comes first in `order_by`.
        GreaterThan = 3,
        /// The given `property` is greater than or equal to the given `value`.
        ///
        /// Requires:
        ///
        /// * That `property` comes first in `order_by`.
        GreaterThanOrEqual = 4,
        /// The given `property` is equal to the given `value`.
        Equal = 5,
        /// The given `property` is equal to at least one value in the given array.
        ///
        /// Requires:
        ///
        /// * That `value` is a non-empty `ArrayValue` with at most 10 values.
        /// * No other `IN` or `NOT_IN` is in the same query.
        In = 6,
        /// The given `property` is not equal to the given `value`.
        ///
        /// Requires:
        ///
        /// * No other `NOT_EQUAL` or `NOT_IN` is in the same query.
        /// * That `property` comes first in the `order_by`.
        NotEqual = 9,
        /// Limit the result set to the given entity and its descendants.
        ///
        /// Requires:
        ///
        /// * That `value` is an entity key.
        HasAncestor = 11,
        /// The value of the `property` is not in the given array.
        ///
        /// Requires:
        ///
        /// * That `value` is a non-empty `ArrayValue` with at most 10 values.
        /// * No other `IN`, `NOT_IN`, `NOT_EQUAL` is in the same query.
        /// * That `field` comes first in the `order_by`.
        NotIn = 13,
    }
}
/// A [GQL
/// query](https://cloud.google.com/datastore/docs/apis/gql/gql_reference).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GqlQuery {
    /// A string of the format described
    /// [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference).
    #[prost(string, tag = "1")]
    pub query_string: ::prost::alloc::string::String,
    /// When false, the query string must not contain any literals and instead must
    /// bind all values. For example,
    /// `SELECT * FROM Kind WHERE a = 'string literal'` is not allowed, while
    /// `SELECT * FROM Kind WHERE a = @value` is.
    #[prost(bool, tag = "2")]
    pub allow_literals: bool,
    /// For each non-reserved named binding site in the query string, there must be
    /// a named parameter with that name, but not necessarily the inverse.
    ///
    /// Key must match regex `[A-Za-z_$][A-Za-z_$0-9]*`, must not match regex
    /// `__.*__`, and must not be `""`.
    #[prost(map = "string, message", tag = "5")]
    pub named_bindings:
        ::std::collections::HashMap<::prost::alloc::string::String, GqlQueryParameter>,
    /// Numbered binding site @1 references the first numbered parameter,
    /// effectively using 1-based indexing, rather than the usual 0.
    ///
    /// For each binding site numbered i in `query_string`, there must be an i-th
    /// numbered parameter. The inverse must also be true.
    #[prost(message, repeated, tag = "4")]
    pub positional_bindings: ::prost::alloc::vec::Vec<GqlQueryParameter>,
}
/// A binding parameter for a GQL query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GqlQueryParameter {
    /// The type of parameter.
    #[prost(oneof = "gql_query_parameter::ParameterType", tags = "2, 3")]
    pub parameter_type: ::core::option::Option<gql_query_parameter::ParameterType>,
}
/// Nested message and enum types in `GqlQueryParameter`.
pub mod gql_query_parameter {
    /// The type of parameter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ParameterType {
        /// A value parameter.
        #[prost(message, tag = "2")]
        Value(super::Value),
        /// A query cursor. Query cursors are returned in query
        /// result batches.
        #[prost(bytes, tag = "3")]
        Cursor(::prost::alloc::vec::Vec<u8>),
    }
}
/// A batch of results produced by a query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResultBatch {
    /// The number of results skipped, typically because of an offset.
    #[prost(int32, tag = "6")]
    pub skipped_results: i32,
    /// A cursor that points to the position after the last skipped result.
    /// Will be set when `skipped_results` != 0.
    #[prost(bytes = "vec", tag = "3")]
    pub skipped_cursor: ::prost::alloc::vec::Vec<u8>,
    /// The result type for every entity in `entity_results`.
    #[prost(enumeration = "entity_result::ResultType", tag = "1")]
    pub entity_result_type: i32,
    /// The results for this batch.
    #[prost(message, repeated, tag = "2")]
    pub entity_results: ::prost::alloc::vec::Vec<EntityResult>,
    /// A cursor that points to the position after the last result in the batch.
    #[prost(bytes = "vec", tag = "4")]
    pub end_cursor: ::prost::alloc::vec::Vec<u8>,
    /// The state of the query after the current batch.
    #[prost(enumeration = "query_result_batch::MoreResultsType", tag = "5")]
    pub more_results: i32,
    /// The version number of the snapshot this batch was returned from.
    /// This applies to the range of results from the query's `start_cursor` (or
    /// the beginning of the query if no cursor was given) to this batch's
    /// `end_cursor` (not the query's `end_cursor`).
    ///
    /// In a single transaction, subsequent query result batches for the same query
    /// can have a greater snapshot version number. Each batch's snapshot version
    /// is valid for all preceding batches.
    /// The value will be zero for eventually consistent queries.
    #[prost(int64, tag = "7")]
    pub snapshot_version: i64,
    /// Read timestamp this batch was returned from.
    /// This applies to the range of results from the query's `start_cursor` (or
    /// the beginning of the query if no cursor was given) to this batch's
    /// `end_cursor` (not the query's `end_cursor`).
    ///
    /// In a single transaction, subsequent query result batches for the same query
    /// can have a greater timestamp. Each batch's read timestamp
    /// is valid for all preceding batches.
    /// This value will not be set for eventually consistent queries in Cloud
    /// Datastore.
    #[prost(message, optional, tag = "8")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `QueryResultBatch`.
pub mod query_result_batch {
    /// The possible values for the `more_results` field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MoreResultsType {
        /// Unspecified. This value is never used.
        Unspecified = 0,
        /// There may be additional batches to fetch from this query.
        NotFinished = 1,
        /// The query is finished, but there may be more results after the limit.
        MoreResultsAfterLimit = 2,
        /// The query is finished, but there may be more results after the end
        /// cursor.
        MoreResultsAfterCursor = 4,
        /// The query is finished, and there are no more results.
        NoMoreResults = 3,
    }
}
/// The request for [Datastore.Lookup][google.datastore.v1.Datastore.Lookup].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct LookupRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// The options for this lookup request.
    #[prost(message, optional, tag = "1")]
    pub read_options: ::core::option::Option<ReadOptions>,
    /// Keys of entities to look up.
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// The response for [Datastore.Lookup][google.datastore.v1.Datastore.Lookup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupResponse {
    /// Entities found as `ResultType.FULL` entities. The order of results in this
    /// field is undefined and has no relation to the order of the keys in the
    /// input.
    #[prost(message, repeated, tag = "1")]
    pub found: ::prost::alloc::vec::Vec<EntityResult>,
    /// Entities not found as `ResultType.KEY_ONLY` entities. The order of results
    /// in this field is undefined and has no relation to the order of the keys
    /// in the input.
    #[prost(message, repeated, tag = "2")]
    pub missing: ::prost::alloc::vec::Vec<EntityResult>,
    /// A list of keys that were not looked up due to resource constraints. The
    /// order of results in this field is undefined and has no relation to the
    /// order of the keys in the input.
    #[prost(message, repeated, tag = "3")]
    pub deferred: ::prost::alloc::vec::Vec<Key>,
}
/// The request for [Datastore.RunQuery][google.datastore.v1.Datastore.RunQuery].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunQueryRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// Entities are partitioned into subsets, identified by a partition ID.
    /// Queries are scoped to a single partition.
    /// This partition ID is normalized with the standard default context
    /// partition ID.
    #[prost(message, optional, tag = "2")]
    pub partition_id: ::core::option::Option<PartitionId>,
    /// The options for this query.
    #[prost(message, optional, tag = "1")]
    pub read_options: ::core::option::Option<ReadOptions>,
    /// The type of query.
    #[prost(oneof = "run_query_request::QueryType", tags = "3, 7")]
    pub query_type: ::core::option::Option<run_query_request::QueryType>,
}
/// Nested message and enum types in `RunQueryRequest`.
pub mod run_query_request {
    /// The type of query.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryType {
        /// The query to run.
        #[prost(message, tag = "3")]
        Query(super::Query),
        /// The GQL query to run.
        #[prost(message, tag = "7")]
        GqlQuery(super::GqlQuery),
    }
}
/// The response for
/// [Datastore.RunQuery][google.datastore.v1.Datastore.RunQuery].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunQueryResponse {
    /// A batch of query results (always present).
    #[prost(message, optional, tag = "1")]
    pub batch: ::core::option::Option<QueryResultBatch>,
    /// The parsed form of the `GqlQuery` from the request, if it was set.
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<Query>,
}
/// The request for
/// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct BeginTransactionRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// Options for a new transaction.
    #[prost(message, optional, tag = "10")]
    pub transaction_options: ::core::option::Option<TransactionOptions>,
}
/// The response for
/// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct BeginTransactionResponse {
    /// The transaction identifier (always present).
    #[prost(bytes = "vec", tag = "1")]
    pub transaction: ::prost::alloc::vec::Vec<u8>,
}
/// The request for [Datastore.Rollback][google.datastore.v1.Datastore.Rollback].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct RollbackRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// The transaction identifier, returned by a call to
    /// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction].
    #[prost(bytes = "vec", tag = "1")]
    pub transaction: ::prost::alloc::vec::Vec<u8>,
}
/// The response for
/// [Datastore.Rollback][google.datastore.v1.Datastore.Rollback]. (an empty
/// message).
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct RollbackResponse {}
/// The request for [Datastore.Commit][google.datastore.v1.Datastore.Commit].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// The type of commit to perform. Defaults to `TRANSACTIONAL`.
    #[prost(enumeration = "commit_request::Mode", tag = "5")]
    pub mode: i32,
    /// The mutations to perform.
    ///
    /// When mode is `TRANSACTIONAL`, mutations affecting a single entity are
    /// applied in order. The following sequences of mutations affecting a single
    /// entity are not permitted in a single `Commit` request:
    ///
    /// - `insert` followed by `insert`
    /// - `update` followed by `insert`
    /// - `upsert` followed by `insert`
    /// - `delete` followed by `update`
    ///
    /// When mode is `NON_TRANSACTIONAL`, no two mutations may affect a single
    /// entity.
    #[prost(message, repeated, tag = "6")]
    pub mutations: ::prost::alloc::vec::Vec<Mutation>,
    /// Must be set when mode is `TRANSACTIONAL`.
    #[prost(oneof = "commit_request::TransactionSelector", tags = "1")]
    pub transaction_selector: ::core::option::Option<commit_request::TransactionSelector>,
}
/// Nested message and enum types in `CommitRequest`.
pub mod commit_request {
    /// The modes available for commits.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Unspecified. This value must not be used.
        Unspecified = 0,
        /// Transactional: The mutations are either all applied, or none are applied.
        /// Learn about transactions
        /// [here](https://cloud.google.com/datastore/docs/concepts/transactions).
        Transactional = 1,
        /// Non-transactional: The mutations may not apply as all or none.
        NonTransactional = 2,
    }
    /// Must be set when mode is `TRANSACTIONAL`.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof)]
    pub enum TransactionSelector {
        /// The identifier of the transaction associated with the commit. A
        /// transaction identifier is returned by a call to
        /// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction].
        #[prost(bytes, tag = "1")]
        Transaction(::prost::alloc::vec::Vec<u8>),
    }
}
/// The response for [Datastore.Commit][google.datastore.v1.Datastore.Commit].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct CommitResponse {
    /// The result of performing the mutations.
    /// The i-th mutation result corresponds to the i-th mutation in the request.
    #[prost(message, repeated, tag = "3")]
    pub mutation_results: ::prost::alloc::vec::Vec<MutationResult>,
    /// The number of index entries updated during the commit, or zero if none were
    /// updated.
    #[prost(int32, tag = "4")]
    pub index_updates: i32,
}
/// The request for
/// [Datastore.AllocateIds][google.datastore.v1.Datastore.AllocateIds].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct AllocateIdsRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// A list of keys with incomplete key paths for which to allocate IDs.
    /// No key may be reserved/read-only.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// The response for
/// [Datastore.AllocateIds][google.datastore.v1.Datastore.AllocateIds].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct AllocateIdsResponse {
    /// The keys specified in the request (in the same order), each with
    /// its key path completed with a newly allocated ID.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// The request for
/// [Datastore.ReserveIds][google.datastore.v1.Datastore.ReserveIds].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct ReserveIdsRequest {
    /// The ID of the project against which to make the request.
    #[prost(string, tag = "8")]
    pub project_id: ::prost::alloc::string::String,
    /// If not empty, the ID of the database against which to make the request.
    #[prost(string, tag = "9")]
    pub database_id: ::prost::alloc::string::String,
    /// A list of keys with complete key paths whose numeric IDs should not be
    /// auto-allocated.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// The response for
/// [Datastore.ReserveIds][google.datastore.v1.Datastore.ReserveIds].
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct ReserveIdsResponse {}
/// A mutation to apply to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    /// The mutation operation.
    ///
    /// For `insert`, `update`, and `upsert`:
    /// - The entity's key must not be reserved/read-only.
    /// - No property in the entity may have a reserved name,
    ///   not even a property in an entity in a value.
    /// - No value in the entity may have meaning 18,
    ///   not even a value in an entity in another value.
    #[prost(oneof = "mutation::Operation", tags = "4, 5, 6, 7")]
    pub operation: ::core::option::Option<mutation::Operation>,
    /// When set, the server will detect whether or not this mutation conflicts
    /// with the current version of the entity on the server. Conflicting mutations
    /// are not applied, and are marked as such in MutationResult.
    #[prost(oneof = "mutation::ConflictDetectionStrategy", tags = "8")]
    pub conflict_detection_strategy: ::core::option::Option<mutation::ConflictDetectionStrategy>,
}
/// Nested message and enum types in `Mutation`.
pub mod mutation {
    /// The mutation operation.
    ///
    /// For `insert`, `update`, and `upsert`:
    /// - The entity's key must not be reserved/read-only.
    /// - No property in the entity may have a reserved name,
    ///   not even a property in an entity in a value.
    /// - No value in the entity may have meaning 18,
    ///   not even a value in an entity in another value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// The entity to insert. The entity must not already exist.
        /// The entity key's final path element may be incomplete.
        #[prost(message, tag = "4")]
        Insert(super::Entity),
        /// The entity to update. The entity must already exist.
        /// Must have a complete key path.
        #[prost(message, tag = "5")]
        Update(super::Entity),
        /// The entity to upsert. The entity may or may not already exist.
        /// The entity key's final path element may be incomplete.
        #[prost(message, tag = "6")]
        Upsert(super::Entity),
        /// The key of the entity to delete. The entity may or may not already exist.
        /// Must have a complete key path and must not be reserved/read-only.
        #[prost(message, tag = "7")]
        Delete(super::Key),
    }
    /// When set, the server will detect whether or not this mutation conflicts
    /// with the current version of the entity on the server. Conflicting mutations
    /// are not applied, and are marked as such in MutationResult.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof)]
    pub enum ConflictDetectionStrategy {
        /// The version of the entity that this mutation is being applied to. If this
        /// does not match the current version on the server, the mutation conflicts.
        #[prost(int64, tag = "8")]
        BaseVersion(i64),
    }
}
/// The result of applying a mutation.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct MutationResult {
    /// The automatically allocated key.
    /// Set only when the mutation allocated a key.
    #[prost(message, optional, tag = "3")]
    pub key: ::core::option::Option<Key>,
    /// The version of the entity on the server after processing the mutation. If
    /// the mutation doesn't change anything on the server, then the version will
    /// be the version of the current entity or, if no entity is present, a version
    /// that is strictly greater than the version of any previous entity and less
    /// than the version of any possible future entity.
    #[prost(int64, tag = "4")]
    pub version: i64,
    /// Whether a conflict was detected for this mutation. Always false when a
    /// conflict detection strategy field is not set in the mutation.
    #[prost(bool, tag = "5")]
    pub conflict_detected: bool,
}
/// The options shared by read requests.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct ReadOptions {
    /// If not specified, lookups and ancestor queries default to
    /// `read_consistency`=`STRONG`, global queries default to
    /// `read_consistency`=`EVENTUAL`.
    #[prost(oneof = "read_options::ConsistencyType", tags = "1, 2")]
    pub consistency_type: ::core::option::Option<read_options::ConsistencyType>,
}
/// Nested message and enum types in `ReadOptions`.
pub mod read_options {
    /// The possible values for read consistencies.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReadConsistency {
        /// Unspecified. This value must not be used.
        Unspecified = 0,
        /// Strong consistency.
        Strong = 1,
        /// Eventual consistency.
        Eventual = 2,
    }
    /// If not specified, lookups and ancestor queries default to
    /// `read_consistency`=`STRONG`, global queries default to
    /// `read_consistency`=`EVENTUAL`.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof)]
    pub enum ConsistencyType {
        /// The non-transactional read consistency to use.
        /// Cannot be set to `STRONG` for global queries.
        #[prost(enumeration = "ReadConsistency", tag = "1")]
        ReadConsistency(i32),
        /// The identifier of the transaction in which to read. A
        /// transaction identifier is returned by a call to
        /// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction].
        #[prost(bytes, tag = "2")]
        Transaction(::prost::alloc::vec::Vec<u8>),
    }
}
/// Options for beginning a new transaction.
///
/// Transactions can be created explicitly with calls to
/// [Datastore.BeginTransaction][google.datastore.v1.Datastore.BeginTransaction]
/// or implicitly by setting
/// [ReadOptions.new_transaction][google.datastore.v1.ReadOptions.new_transaction]
/// in read requests.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct TransactionOptions {
    /// The `mode` of the transaction, indicating whether write operations are
    /// supported.
    #[prost(oneof = "transaction_options::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<transaction_options::Mode>,
}
/// Nested message and enum types in `TransactionOptions`.
pub mod transaction_options {
    /// Options specific to read / write transactions.
    #[derive(Clone, PartialEq, Eq, ::prost::Message)]
    pub struct ReadWrite {
        /// The transaction identifier of the transaction being retried.
        #[prost(bytes = "vec", tag = "1")]
        pub previous_transaction: ::prost::alloc::vec::Vec<u8>,
    }
    /// Options specific to read-only transactions.
    #[derive(Clone, PartialEq, Eq, ::prost::Message)]
    pub struct ReadOnly {}
    /// The `mode` of the transaction, indicating whether write operations are
    /// supported.
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof)]
    pub enum Mode {
        /// The transaction should allow both reads and writes.
        #[prost(message, tag = "1")]
        ReadWrite(ReadWrite),
        /// The transaction should only allow reads.
        #[prost(message, tag = "2")]
        ReadOnly(ReadOnly),
    }
}
#[doc = r" Generated client implementations."]
pub mod datastore_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Each RPC normalizes the partition IDs of the keys in its input entities,"]
    #[doc = " and always returns entities with keys with normalized partition IDs."]
    #[doc = " This applies to all keys and entities, including those in values, except keys"]
    #[doc = " with both an empty path and an empty or unset partition ID. Normalization of"]
    #[doc = " input keys sets the project ID (if not already set) to the project ID from"]
    #[doc = " the request."]
    #[doc = ""]
    pub struct DatastoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatastoreClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatastoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Looks up entities by key."]
        pub async fn lookup(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupRequest>,
        ) -> Result<tonic::Response<super::LookupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/Lookup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Queries for entities."]
        pub async fn run_query(
            &mut self,
            request: impl tonic::IntoRequest<super::RunQueryRequest>,
        ) -> Result<tonic::Response<super::RunQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/RunQuery");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Begins a new transaction."]
        pub async fn begin_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::BeginTransactionRequest>,
        ) -> Result<tonic::Response<super::BeginTransactionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.datastore.v1.Datastore/BeginTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Commits a transaction, optionally creating, deleting or modifying some"]
        #[doc = " entities."]
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitRequest>,
        ) -> Result<tonic::Response<super::CommitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/Commit");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rolls back a transaction."]
        pub async fn rollback(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackRequest>,
        ) -> Result<tonic::Response<super::RollbackResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/Rollback");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allocates IDs for the given keys, which is useful for referencing an entity"]
        #[doc = " before it is inserted."]
        pub async fn allocate_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::AllocateIdsRequest>,
        ) -> Result<tonic::Response<super::AllocateIdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/AllocateIds");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Prevents the supplied keys' IDs from being auto-allocated by Cloud"]
        #[doc = " Datastore."]
        pub async fn reserve_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveIdsRequest>,
        ) -> Result<tonic::Response<super::ReserveIdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.datastore.v1.Datastore/ReserveIds");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatastoreClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatastoreClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatastoreClient {{ ... }}")
        }
    }
}
