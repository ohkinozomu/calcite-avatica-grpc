/// Details about a connection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProperties {
    #[prost(bool, tag="1")]
    pub is_dirty: bool,
    #[prost(bool, tag="2")]
    pub auto_commit: bool,
    /// field is a Boolean, need to discern null and default value
    #[prost(bool, tag="7")]
    pub has_auto_commit: bool,
    #[prost(bool, tag="3")]
    pub read_only: bool,
    /// field is a Boolean, need to discern null and default value
    #[prost(bool, tag="8")]
    pub has_read_only: bool,
    #[prost(uint32, tag="4")]
    pub transaction_isolation: u32,
    #[prost(string, tag="5")]
    pub catalog: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub schema: ::prost::alloc::string::String,
}
/// Statement handle
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatementHandle {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub id: u32,
    #[prost(message, optional, tag="3")]
    pub signature: ::core::option::Option<Signature>,
}
/// Results of preparing a statement
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(message, repeated, tag="1")]
    pub columns: ::prost::alloc::vec::Vec<ColumnMetaData>,
    #[prost(string, tag="2")]
    pub sql: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub parameters: ::prost::alloc::vec::Vec<AvaticaParameter>,
    #[prost(message, optional, tag="4")]
    pub cursor_factory: ::core::option::Option<CursorFactory>,
    #[prost(enumeration="StatementType", tag="5")]
    pub statement_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnMetaData {
    #[prost(uint32, tag="1")]
    pub ordinal: u32,
    #[prost(bool, tag="2")]
    pub auto_increment: bool,
    #[prost(bool, tag="3")]
    pub case_sensitive: bool,
    #[prost(bool, tag="4")]
    pub searchable: bool,
    #[prost(bool, tag="5")]
    pub currency: bool,
    #[prost(uint32, tag="6")]
    pub nullable: u32,
    #[prost(bool, tag="7")]
    pub signed: bool,
    #[prost(uint32, tag="8")]
    pub display_size: u32,
    #[prost(string, tag="9")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub column_name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="12")]
    pub precision: u32,
    #[prost(uint32, tag="13")]
    pub scale: u32,
    #[prost(string, tag="14")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub catalog_name: ::prost::alloc::string::String,
    #[prost(bool, tag="16")]
    pub read_only: bool,
    #[prost(bool, tag="17")]
    pub writable: bool,
    #[prost(bool, tag="18")]
    pub definitely_writable: bool,
    #[prost(string, tag="19")]
    pub column_class_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="20")]
    pub r#type: ::core::option::Option<AvaticaType>,
}
/// Base class for a column type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvaticaType {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Rep", tag="3")]
    pub rep: i32,
    /// Only present when name = STRUCT
    #[prost(message, repeated, tag="4")]
    pub columns: ::prost::alloc::vec::Vec<ColumnMetaData>,
    /// Only present when name = ARRAY
    #[prost(message, optional, boxed, tag="5")]
    pub component: ::core::option::Option<::prost::alloc::boxed::Box<AvaticaType>>,
}
/// Metadata for a parameter
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvaticaParameter {
    #[prost(bool, tag="1")]
    pub signed: bool,
    #[prost(uint32, tag="2")]
    pub precision: u32,
    #[prost(uint32, tag="3")]
    pub scale: u32,
    #[prost(uint32, tag="4")]
    pub parameter_type: u32,
    #[prost(string, tag="5")]
    pub type_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
}
/// Information necessary to convert an Iterable into a Calcite Cursor
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorFactory {
    #[prost(enumeration="cursor_factory::Style", tag="1")]
    pub style: i32,
    #[prost(string, tag="2")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `CursorFactory`.
pub mod cursor_factory {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Style {
        Object = 0,
        Record = 1,
        RecordProjection = 2,
        Array = 3,
        List = 4,
        Map = 5,
    }
    impl Style {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Style::Object => "OBJECT",
                Style::Record => "RECORD",
                Style::RecordProjection => "RECORD_PROJECTION",
                Style::Array => "ARRAY",
                Style::List => "LIST",
                Style::Map => "MAP",
            }
        }
    }
}
/// A collection of rows
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(uint64, tag="1")]
    pub offset: u64,
    #[prost(bool, tag="2")]
    pub done: bool,
    #[prost(message, repeated, tag="3")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
/// A row is a collection of values
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(message, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<ColumnValue>,
}
/// Database property, list of functions the database provides for a certain operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseProperty {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub functions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message which encapsulates another message to support a single RPC endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WireMessage {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub wrapped_message: ::prost::alloc::vec::Vec<u8>,
}
/// A value might be a TypedValue or an Array of TypedValue's
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnValue {
    /// deprecated, use array_value or scalar_value
    #[prost(message, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<TypedValue>,
    #[prost(message, repeated, tag="2")]
    pub array_value: ::prost::alloc::vec::Vec<TypedValue>,
    /// Is an array value set?
    #[prost(bool, tag="3")]
    pub has_array_value: bool,
    #[prost(message, optional, tag="4")]
    pub scalar_value: ::core::option::Option<TypedValue>,
}
/// Generic wrapper to support any SQL type. Struct-like to work around no polymorphism construct.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The actual type that was serialized in the general attribute below
    #[prost(enumeration="Rep", tag="1")]
    pub r#type: i32,
    /// boolean
    #[prost(bool, tag="2")]
    pub bool_value: bool,
    /// char/varchar
    #[prost(string, tag="3")]
    pub string_value: ::prost::alloc::string::String,
    /// var-len encoding lets us shove anything from byte to long
    #[prost(sint64, tag="4")]
    pub number_value: i64,
    /// includes numeric types and date/time types.
    ///
    /// binary/varbinary
    #[prost(bytes="vec", tag="5")]
    pub bytes_value: ::prost::alloc::vec::Vec<u8>,
    /// big numbers
    #[prost(double, tag="6")]
    pub double_value: f64,
    /// a null object
    #[prost(bool, tag="7")]
    pub null: bool,
    /// The Array
    #[prost(message, repeated, tag="8")]
    pub array_value: ::prost::alloc::vec::Vec<TypedValue>,
    /// If an Array, the representation for the array values
    #[prost(enumeration="Rep", tag="9")]
    pub component_type: i32,
    /// Differentiate between explicitly null (user-set) and implicitly null
    #[prost(bool, tag="10")]
    pub implicitly_null: bool,
}
/// Represents the breadth of arguments to DatabaseMetaData functions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaDataOperationArgument {
    #[prost(string, tag="1")]
    pub string_value: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub bool_value: bool,
    #[prost(sint32, tag="3")]
    pub int_value: i32,
    #[prost(string, repeated, tag="4")]
    pub string_array_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint32, repeated, tag="5")]
    pub int_array_values: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="meta_data_operation_argument::ArgumentType", tag="6")]
    pub r#type: i32,
}
/// Nested message and enum types in `MetaDataOperationArgument`.
pub mod meta_data_operation_argument {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ArgumentType {
        String = 0,
        Bool = 1,
        Int = 2,
        RepeatedString = 3,
        RepeatedInt = 4,
        Null = 5,
    }
    impl ArgumentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ArgumentType::String => "STRING",
                ArgumentType::Bool => "BOOL",
                ArgumentType::Int => "INT",
                ArgumentType::RepeatedString => "REPEATED_STRING",
                ArgumentType::RepeatedInt => "REPEATED_INT",
                ArgumentType::Null => "NULL",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryState {
    #[prost(enumeration="StateType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub sql: ::prost::alloc::string::String,
    #[prost(enumeration="MetaDataOperation", tag="3")]
    pub op: i32,
    #[prost(message, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<MetaDataOperationArgument>,
    #[prost(bool, tag="5")]
    pub has_args: bool,
    #[prost(bool, tag="6")]
    pub has_sql: bool,
    #[prost(bool, tag="7")]
    pub has_op: bool,
}
/// Has to be consistent with Meta.StatementType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatementType {
    Select = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
    Upsert = 4,
    Merge = 5,
    OtherDml = 6,
    Create = 7,
    Drop = 8,
    Alter = 9,
    OtherDdl = 10,
    Call = 11,
}
impl StatementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatementType::Select => "SELECT",
            StatementType::Insert => "INSERT",
            StatementType::Update => "UPDATE",
            StatementType::Delete => "DELETE",
            StatementType::Upsert => "UPSERT",
            StatementType::Merge => "MERGE",
            StatementType::OtherDml => "OTHER_DML",
            StatementType::Create => "CREATE",
            StatementType::Drop => "DROP",
            StatementType::Alter => "ALTER",
            StatementType::OtherDdl => "OTHER_DDL",
            StatementType::Call => "CALL",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Rep {
    PrimitiveBoolean = 0,
    PrimitiveByte = 1,
    PrimitiveChar = 2,
    PrimitiveShort = 3,
    PrimitiveInt = 4,
    PrimitiveLong = 5,
    PrimitiveFloat = 6,
    PrimitiveDouble = 7,
    Boolean = 8,
    Byte = 9,
    Character = 10,
    Short = 11,
    Integer = 12,
    Long = 13,
    Float = 14,
    Double = 15,
    BigInteger = 25,
    BigDecimal = 26,
    JavaSqlTime = 16,
    JavaSqlTimestamp = 17,
    JavaSqlDate = 18,
    JavaUtilDate = 19,
    ByteString = 20,
    String = 21,
    Number = 22,
    Object = 23,
    Null = 24,
    Array = 27,
    Struct = 28,
    Multiset = 29,
}
impl Rep {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rep::PrimitiveBoolean => "PRIMITIVE_BOOLEAN",
            Rep::PrimitiveByte => "PRIMITIVE_BYTE",
            Rep::PrimitiveChar => "PRIMITIVE_CHAR",
            Rep::PrimitiveShort => "PRIMITIVE_SHORT",
            Rep::PrimitiveInt => "PRIMITIVE_INT",
            Rep::PrimitiveLong => "PRIMITIVE_LONG",
            Rep::PrimitiveFloat => "PRIMITIVE_FLOAT",
            Rep::PrimitiveDouble => "PRIMITIVE_DOUBLE",
            Rep::Boolean => "BOOLEAN",
            Rep::Byte => "BYTE",
            Rep::Character => "CHARACTER",
            Rep::Short => "SHORT",
            Rep::Integer => "INTEGER",
            Rep::Long => "LONG",
            Rep::Float => "FLOAT",
            Rep::Double => "DOUBLE",
            Rep::BigInteger => "BIG_INTEGER",
            Rep::BigDecimal => "BIG_DECIMAL",
            Rep::JavaSqlTime => "JAVA_SQL_TIME",
            Rep::JavaSqlTimestamp => "JAVA_SQL_TIMESTAMP",
            Rep::JavaSqlDate => "JAVA_SQL_DATE",
            Rep::JavaUtilDate => "JAVA_UTIL_DATE",
            Rep::ByteString => "BYTE_STRING",
            Rep::String => "STRING",
            Rep::Number => "NUMBER",
            Rep::Object => "OBJECT",
            Rep::Null => "NULL",
            Rep::Array => "ARRAY",
            Rep::Struct => "STRUCT",
            Rep::Multiset => "MULTISET",
        }
    }
}
/// The severity of some unexpected outcome to an operation.
/// Protobuf enum values must be unique across all other enums
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Severity {
    UnknownSeverity = 0,
    FatalSeverity = 1,
    ErrorSeverity = 2,
    WarningSeverity = 3,
}
impl Severity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Severity::UnknownSeverity => "UNKNOWN_SEVERITY",
            Severity::FatalSeverity => "FATAL_SEVERITY",
            Severity::ErrorSeverity => "ERROR_SEVERITY",
            Severity::WarningSeverity => "WARNING_SEVERITY",
        }
    }
}
/// Enumeration corresponding to DatabaseMetaData operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetaDataOperation {
    GetAttributes = 0,
    GetBestRowIdentifier = 1,
    GetCatalogs = 2,
    GetClientInfoProperties = 3,
    GetColumnPrivileges = 4,
    GetColumns = 5,
    GetCrossReference = 6,
    GetExportedKeys = 7,
    GetFunctionColumns = 8,
    GetFunctions = 9,
    GetImportedKeys = 10,
    GetIndexInfo = 11,
    GetPrimaryKeys = 12,
    GetProcedureColumns = 13,
    GetProcedures = 14,
    GetPseudoColumns = 15,
    GetSchemas = 16,
    GetSchemasWithArgs = 17,
    GetSuperTables = 18,
    GetSuperTypes = 19,
    GetTablePrivileges = 20,
    GetTables = 21,
    GetTableTypes = 22,
    GetTypeInfo = 23,
    GetUdts = 24,
    GetVersionColumns = 25,
}
impl MetaDataOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MetaDataOperation::GetAttributes => "GET_ATTRIBUTES",
            MetaDataOperation::GetBestRowIdentifier => "GET_BEST_ROW_IDENTIFIER",
            MetaDataOperation::GetCatalogs => "GET_CATALOGS",
            MetaDataOperation::GetClientInfoProperties => "GET_CLIENT_INFO_PROPERTIES",
            MetaDataOperation::GetColumnPrivileges => "GET_COLUMN_PRIVILEGES",
            MetaDataOperation::GetColumns => "GET_COLUMNS",
            MetaDataOperation::GetCrossReference => "GET_CROSS_REFERENCE",
            MetaDataOperation::GetExportedKeys => "GET_EXPORTED_KEYS",
            MetaDataOperation::GetFunctionColumns => "GET_FUNCTION_COLUMNS",
            MetaDataOperation::GetFunctions => "GET_FUNCTIONS",
            MetaDataOperation::GetImportedKeys => "GET_IMPORTED_KEYS",
            MetaDataOperation::GetIndexInfo => "GET_INDEX_INFO",
            MetaDataOperation::GetPrimaryKeys => "GET_PRIMARY_KEYS",
            MetaDataOperation::GetProcedureColumns => "GET_PROCEDURE_COLUMNS",
            MetaDataOperation::GetProcedures => "GET_PROCEDURES",
            MetaDataOperation::GetPseudoColumns => "GET_PSEUDO_COLUMNS",
            MetaDataOperation::GetSchemas => "GET_SCHEMAS",
            MetaDataOperation::GetSchemasWithArgs => "GET_SCHEMAS_WITH_ARGS",
            MetaDataOperation::GetSuperTables => "GET_SUPER_TABLES",
            MetaDataOperation::GetSuperTypes => "GET_SUPER_TYPES",
            MetaDataOperation::GetTablePrivileges => "GET_TABLE_PRIVILEGES",
            MetaDataOperation::GetTables => "GET_TABLES",
            MetaDataOperation::GetTableTypes => "GET_TABLE_TYPES",
            MetaDataOperation::GetTypeInfo => "GET_TYPE_INFO",
            MetaDataOperation::GetUdts => "GET_UDTS",
            MetaDataOperation::GetVersionColumns => "GET_VERSION_COLUMNS",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StateType {
    Sql = 0,
    Metadata = 1,
}
impl StateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StateType::Sql => "SQL",
            StateType::Metadata => "METADATA",
        }
    }
}
/// Request for Meta#getCatalogs()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogsRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request for Meta#getDatabaseProperties()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabasePropertyRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request for Meta#getSchemas(String, org.apache.calcite.avatica.Meta.Pat)}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemasRequest {
    #[prost(string, tag="1")]
    pub catalog: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schema_pattern: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub has_catalog: bool,
    #[prost(bool, tag="5")]
    pub has_schema_pattern: bool,
}
/// Request for Request for Meta#getTables(String, org.apache.calcite.avatica.Meta.Pat,
///    org.apache.calcite.avatica.Meta.Pat, java.util.List)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TablesRequest {
    #[prost(string, tag="1")]
    pub catalog: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schema_pattern: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name_pattern: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub type_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Having an empty type_list is distinct from a null type_list
    #[prost(bool, tag="6")]
    pub has_type_list: bool,
    #[prost(string, tag="7")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub has_catalog: bool,
    #[prost(bool, tag="9")]
    pub has_schema_pattern: bool,
    #[prost(bool, tag="10")]
    pub has_table_name_pattern: bool,
}
/// Request for Meta#getTableTypes()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableTypesRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request for Meta#getColumns(String, org.apache.calcite.avatica.Meta.Pat,
///    org.apache.calcite.avatica.Meta.Pat, org.apache.calcite.avatica.Meta.Pat).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnsRequest {
    #[prost(string, tag="1")]
    pub catalog: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schema_pattern: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub table_name_pattern: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub column_name_pattern: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub has_catalog: bool,
    #[prost(bool, tag="7")]
    pub has_schema_pattern: bool,
    #[prost(bool, tag="8")]
    pub has_table_name_pattern: bool,
    #[prost(bool, tag="9")]
    pub has_column_name_pattern: bool,
}
/// Request for Meta#getTypeInfo()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeInfoRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request for Meta#prepareAndExecute(Meta.StatementHandle, String, long, Meta.PrepareCallback)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareAndExecuteRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sql: ::prost::alloc::string::String,
    /// Deprecated
    #[prost(uint64, tag="3")]
    pub max_row_count: u64,
    #[prost(uint32, tag="4")]
    pub statement_id: u32,
    /// The maximum number of rows that will be allowed for this query
    #[prost(int64, tag="5")]
    pub max_rows_total: i64,
    /// The maximum number of rows that will be returned in the
    #[prost(int32, tag="6")]
    pub first_frame_max_size: i32,
}
/// Request for Meta.prepare(Meta.ConnectionHandle, String, long)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sql: ::prost::alloc::string::String,
    /// Deprecated
    #[prost(uint64, tag="3")]
    pub max_row_count: u64,
    /// The maximum number of rows that will be allowed for this query
    #[prost(int64, tag="4")]
    pub max_rows_total: i64,
}
/// Request for Meta#fetch(Meta.StatementHandle, List, long, int)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(uint64, tag="3")]
    pub offset: u64,
    /// Maximum number of rows to be returned in the frame. Negative means no limit. Deprecated!
    #[prost(uint32, tag="4")]
    pub fetch_max_row_count: u32,
    #[prost(int32, tag="5")]
    pub frame_max_size: i32,
}
/// Request for Meta#createStatement(Meta.ConnectionHandle)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatementRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request for Meta#closeStatement(Meta.StatementHandle)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseStatementRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
}
/// Request for Meta#openConnection(Meta.ConnectionHandle, Map<String, String>)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenConnectionRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="2")]
    pub info: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Request for Meta#closeConnection(Meta.ConnectionHandle)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseConnectionRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionSyncRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub conn_props: ::core::option::Option<ConnectionProperties>,
}
/// Request for Meta#execute(Meta.ConnectionHandle, list, long)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRequest {
    #[prost(message, optional, tag="1")]
    pub statement_handle: ::core::option::Option<StatementHandle>,
    #[prost(message, repeated, tag="2")]
    pub parameter_values: ::prost::alloc::vec::Vec<TypedValue>,
    /// Deprecated, use the signed int instead.
    #[prost(uint64, tag="3")]
    pub deprecated_first_frame_max_size: u64,
    #[prost(bool, tag="4")]
    pub has_parameter_values: bool,
    /// The maximum number of rows to return in the first Frame
    #[prost(int32, tag="5")]
    pub first_frame_max_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResultsRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(message, optional, tag="3")]
    pub state: ::core::option::Option<QueryState>,
    #[prost(uint64, tag="4")]
    pub offset: u64,
}
/// Request to invoke a commit on a Connection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request to invoke rollback on a Connection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Request to prepare and execute a collection of sql statements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareAndExecuteBatchRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(string, repeated, tag="3")]
    pub sql_commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Each command is a list of TypedValues
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBatch {
    #[prost(message, repeated, tag="1")]
    pub parameter_values: ::prost::alloc::vec::Vec<TypedValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteBatchRequest {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    /// A batch of updates is a list<list<typevalue>>
    #[prost(message, repeated, tag="3")]
    pub updates: ::prost::alloc::vec::Vec<UpdateBatch>,
}
/// Response that contains a result set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultSetResponse {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(bool, tag="3")]
    pub own_statement: bool,
    #[prost(message, optional, tag="4")]
    pub signature: ::core::option::Option<Signature>,
    #[prost(message, optional, tag="5")]
    pub first_frame: ::core::option::Option<Frame>,
    /// -1 for normal result sets, else this response contains a dummy result set
    #[prost(uint64, tag="6")]
    pub update_count: u64,
    /// with no signature nor other data.
    #[prost(message, optional, tag="7")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to PrepareAndExecuteRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteResponse {
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<ResultSetResponse>,
    /// Did the request fail because of no-cached statement
    #[prost(bool, tag="2")]
    pub missing_statement: bool,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to PrepareRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareResponse {
    #[prost(message, optional, tag="1")]
    pub statement: ::core::option::Option<StatementHandle>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to FetchRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchResponse {
    #[prost(message, optional, tag="1")]
    pub frame: ::core::option::Option<Frame>,
    /// Did the request fail because of no-cached statement
    #[prost(bool, tag="2")]
    pub missing_statement: bool,
    /// Did the request fail because of a cached-statement w/o ResultSet
    #[prost(bool, tag="3")]
    pub missing_results: bool,
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to CreateStatementRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatementResponse {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to CloseStatementRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseStatementResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to OpenConnectionRequest {
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenConnectionResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to CloseConnectionRequest {
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseConnectionResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response to ConnectionSyncRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionSyncResponse {
    #[prost(message, optional, tag="1")]
    pub conn_props: ::core::option::Option<ConnectionProperties>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabasePropertyElement {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<DatabaseProperty>,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<TypedValue>,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Response for Meta#getDatabaseProperties()
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabasePropertyResponse {
    #[prost(message, repeated, tag="1")]
    pub props: ::prost::alloc::vec::Vec<DatabasePropertyElement>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Send contextual information about some error over the wire from the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    /// exception stacktraces, many for linked exceptions.
    #[prost(string, repeated, tag="1")]
    pub exceptions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// are there stacktraces contained?
    #[prost(bool, tag="7")]
    pub has_exceptions: bool,
    /// human readable description
    #[prost(string, tag="2")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(enumeration="Severity", tag="3")]
    pub severity: i32,
    /// numeric identifier for error
    #[prost(uint32, tag="4")]
    pub error_code: u32,
    /// five-character standard-defined value
    #[prost(string, tag="5")]
    pub sql_state: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResultsResponse {
    /// Server doesn't have the statement with the ID from the request
    #[prost(bool, tag="1")]
    pub missing_statement: bool,
    /// Should the client fetch() to get more results
    #[prost(bool, tag="2")]
    pub more_results: bool,
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
/// Generic metadata for the server to return with each response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcMetadata {
    /// The host:port of the server
    #[prost(string, tag="1")]
    pub server_address: ::prost::alloc::string::String,
}
/// Response to a commit request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
}
/// Response to a rollback request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackResponse {
}
/// Response to a batch update request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteBatchResponse {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub statement_id: u32,
    #[prost(uint64, repeated, tag="3")]
    pub update_counts: ::prost::alloc::vec::Vec<u64>,
    /// Did the request fail because of no-cached statement
    #[prost(bool, tag="4")]
    pub missing_statement: bool,
    #[prost(message, optional, tag="5")]
    pub metadata: ::core::option::Option<RpcMetadata>,
}
