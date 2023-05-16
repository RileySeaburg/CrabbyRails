// Add this import at the top of the `database.rs` file

use std::{collections::HashMap, fs};
use toml::Value;

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub struct Database {
    pub name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database_type: DatabaseType,
}
/// # Name: Database
/// ## Type: Struct
/// ## Description
/// This struct is used to configure the database.
/// This is used when creating a new project and the u
/// Example:
/// ```rust
/// use crate::database::Database;
/// let database = Database::new(
///    "database_name".to_string(),
///   "username".to_string(),
///  "password".to_string(),
/// "host".to_string(),
/// "port".to_string(),
/// "database_type".to_string(),
/// );
/// ```
///
impl Database {
    pub fn new(
        name: String,
        username: String,
        password: String,
        host: String,
        port: String,
        database_type: String,
    ) -> Database {
        Database {
            name,
            username,
            password,
            host,
            port,
            database_type: match database_type.as_str() {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::Mysql,
                "sqlite" => DatabaseType::Sqlite,
                "mongo" => DatabaseType::Mongo,
                // this is defaulting, need to address the code running this line
                _ => DatabaseType::Postgres,
            },
        }
    }

    /// Reads the `rustyroad.toml` configuration file and extracts the database configuration.
    /// Returns a `Database` struct containing the database configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(Database)` - If the `rustyroad.toml` file is found and successfully parsed, returns a `Database` struct.
    /// * `Err(std::io::Error)` - If there is an error reading the `rustyroad.toml` file, returns an I/O error.
    ///
    /// # Panics
    ///
    /// * If the `rustyroad.toml` file is not found or cannot be parsed, the function will print an error message
    ///   and exit the process with status code 1.
    pub fn get_database_from_rustyroad_toml() -> Result<Database, std::io::Error> {
        let database = match fs::read_to_string("rustyroad.toml") {
            Ok(file) => {
                let toml: Value = toml::from_str(&file).unwrap();

                // Access the [database] table from the TOML document
                let database_table = toml["database"].as_table().unwrap();

                // Access the keys within the [database] table
                let name = database_table["database_name"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let username = database_table["database_user"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let password = database_table["database_password"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let host = database_table["database_host"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let port = database_table["database_port"]
                    .as_str()
                    .unwrap()
                    .to_string();
                let database_type = database_table["database_type"]
                    .as_str()
                    .unwrap()
                    .to_string();

                Database::new(name, username, password, host, port, database_type)
            }
            Err(_) => {
                println!("No rustyroad.toml file found in the workspace root.");
                println!("Please run `rustyroad new` to create a new project.");
                std::process::exit(1);
            }
        };
        Ok(database)
    }
}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq)]
pub enum DatabaseType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
}

pub enum DataTypeCategory {
    Numeric,
    DateTime,
    Text,
    Geometric,
    NetworkAddress,
    Json,
    Search,
    Array,
    UUID,
    Monetary,
    BitString,
    Interval,
    Composite,
    Range,
    Other,
}

impl DataTypeCategory {
    pub fn get_data_types_from_data_type_category(
        &self,
        database_type: DatabaseType,
    ) -> DatabaseTypes {
        match database_type {
            // need to map the control flow to the database types
            DatabaseType::Mysql => match &self {
                DataTypeCategory::Numeric => DatabaseTypes::MySqlTypes(vec![
                    MySqlTypes::TinyInt,
                    MySqlTypes::SmallInt,
                    MySqlTypes::MediumInt,
                    MySqlTypes::Int,
                    MySqlTypes::BigInt,
                    MySqlTypes::Decimal,
                    MySqlTypes::Float,
                    MySqlTypes::Double,
                    MySqlTypes::Bit,
                    MySqlTypes::Boolean,
                    MySqlTypes::Serial,
                ]),
                DataTypeCategory::DateTime => todo!(),
                DataTypeCategory::Text => todo!(),
                DataTypeCategory::Geometric => todo!(),
                DataTypeCategory::NetworkAddress => todo!(),
                DataTypeCategory::Json => todo!(),
                DataTypeCategory::Search => todo!(),
                DataTypeCategory::Array => todo!(),
                DataTypeCategory::UUID => todo!(),
                DataTypeCategory::Monetary => todo!(),
                DataTypeCategory::BitString => todo!(),
                DataTypeCategory::Interval => todo!(),
                DataTypeCategory::Composite => todo!(),
                DataTypeCategory::Range => todo!(),
                DataTypeCategory::Other => todo!(),
            },
        }
    }
}

pub enum PostgresTypes {
    Boolean,
    Serial,
    Uuid,
    Array,
    Json,
    JsonB,
    HStore,
    TsVector,
    TsQuery,
    CiText,
    Point,
    Line,
    Lseg,
    Box,
    Path,
    Polygon,
    Circle,
    Inet,
    Cidr,
    Money,
    Bit,
    BitVarying,
    Interval,
    TimeWithTimeZone,
    TimestampWithTimeZone,
    Enum,
    Composite,
    Domain,
    Range,
}

pub struct PostgresTypeMap {
    pub types: HashMap<String, PostgresType>,
}

impl PostgresType {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            PostgresType::Boolean
            | PostgresType::Serial
            | PostgresType::Bit
            | PostgresType::BitVarying
            | PostgresType::Money => DataTypeCategory::Numeric,

            PostgresType::TimeWithTimeZone
            | PostgresType::TimestampWithTimeZone
            | PostgresType::Interval => DataTypeCategory::DateTime,

            PostgresType::Json
            | PostgresType::JsonB
            | PostgresType::HStore
            | PostgresType::CiText
            | PostgresType::Enum => DataTypeCategory::Text,

            PostgresType::Point
            | PostgresType::Line
            | PostgresType::Lseg
            | PostgresType::Box
            | PostgresType::Path
            | PostgresType::Polygon
            | PostgresType::Circle => DataTypeCategory::Geometric,

            PostgresType::Inet | PostgresType::Cidr => DataTypeCategory::NetworkAddress,

            PostgresType::TsVector | PostgresType::TsQuery => DataTypeCategory::Search,

            PostgresType::Array => DataTypeCategory::Array,

            PostgresType::Uuid => DataTypeCategory::UUID,

            PostgresType::Composite => DataTypeCategory::Composite,

            PostgresType::Range => DataTypeCategory::Range,

            PostgresType::Domain => DataTypeCategory::Other,
        }
    }
}

pub enum MySqlTypes {
    Bit,
    TinyInt,
    SmallInt,
    MediumInt,
    Int,
    BigInt,
    Float,
    Double,
    Decimal,
    Date,
    DateTime,
    Time,
    Timestamp,
    Year,
    Char,
    VarChar,
    Binary,
    VarBinary,
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
    TinyText,
    Text,
    MediumText,
    LongText,
    Enum,
    Set,
    Geometry,
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
    Json,
    BinaryJson,
    BitField,
    NewDecimal,
    EnumInner,
    SetInner,
    GeometryInner,
    Unknown,
}

pub struct MySqlTypeMap {
    pub types: HashMap<String, MysqlType>,
}

impl MySqlTypes {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            MySqlTypes::Bit => DataTypeCategory::BitString,
            MySqlTypes::TinyInt
            | MySqlTypes::SmallInt
            | MySqlTypes::MediumInt
            | MySqlTypes::Int
            | MySqlTypes::BigInt
            | MySqlTypes::Float
            | MySqlTypes::Double
            | MySqlTypes::Decimal
            | MySqlTypes::NewDecimal => DataTypeCategory::Numeric,
            MySqlTypes::Date
            | MySqlTypes::DateTime
            | MySqlTypes::Time
            | MySqlTypes::Timestamp
            | MySqlTypes::Year => DataTypeCategory::DateTime,
            MySqlTypes::Char
            | MySqlTypes::VarChar
            | MySqlTypes::Binary
            | MySqlTypes::VarBinary
            | MySqlTypes::TinyBlob
            | MySqlTypes::Blob
            | MySqlTypes::MediumBlob
            | MySqlTypes::LongBlob
            | MySqlTypes::TinyText
            | MySqlTypes::Text
            | MySqlTypes::MediumText
            | MySqlTypes::LongText
            | MySqlTypes::Enum
            | MySqlTypes::Set
            | MySqlTypes::Json
            | MySqlTypes::BinaryJson => DataTypeCategory::Text,
            MySqlTypes::Geometry
            | MySqlTypes::Point
            | MySqlTypes::LineString
            | MySqlTypes::Polygon
            | MySqlTypes::MultiPoint
            | MySqlTypes::MultiLineString
            | MySqlTypes::MultiPolygon
            | MySqlTypes::GeometryCollection => DataTypeCategory::Geometric,
            MySqlTypes::BitField => DataTypeCategory::Other,
            MySqlTypes::EnumInner => DataTypeCategory::Other,
            MySqlTypes::SetInner => DataTypeCategory::Other,
            MySqlTypes::GeometryInner => DataTypeCategory::Other,
            MySqlTypes::Unknown => DataTypeCategory::Other,
        }
    }
}

pub enum SqliteTypes {
    Integer,
    Real,
    Text,
    Blob,
    Numeric,
    Date,
    Time,
    DateTime,
    Boolean,
    Unknown,
}

pub struct SqliteTypeMap {
    types_by_category: HashMap<DataTypeCategory, Vec<PostgresType>>,
}

impl SqliteTypes {
    pub fn category(&self) -> DataTypeCategory {
        match self {
            SqliteType::Integer | SqliteType::Real | SqliteType::Numeric => {
                DataTypeCategory::Numeric
            }
            SqliteType::Date | SqliteType::Time | SqliteType::DateTime => {
                DataTypeCategory::DateTime
            }
            SqliteType::Text => DataTypeCategory::Text,
            SqliteType::Blob => DataTypeCategory::Other,
            SqliteType::Boolean => DataTypeCategory::Other,
            SqliteType::Unknown => DataTypeCategory::Other,
        }
    }
}

pub struct DatabaseTypes {
    pub postgres: PostgresTypeMap,
    pub mysql: MySqlTypeMap,
    pub sqlite: SqliteTypeMap,
}

impl DatabaseTypes {
    pub fn new() -> Self {
        Self {
            postgres: PostgresTypeMap::new(),
            mysql: MysqlTypeMap::new(),
            sqlite: SqliteTypeMap::new(),
        }
    }

    pub fn add_postgres_type(&mut self, ty: PostgresTypes) {
        let category = ty.category();
        self.postgres
            .entry(category)
            .or_insert_with(Vec::new)
            .push(ty);
    }

    pub fn add_mysql_type(&mut self, ty: MySqlTypes) {
        let category = ty.category();
        self.mysql.entry(category).or_insert_with(Vec::new).push(ty);
    }

    pub fn add_sqlite_type(&mut self, ty: SqliteTypes) {
        let category = ty.category();
        self.sqlite
            .entry(category)
            .or_insert_with(Vec::new)
            .push(ty);
    }

    pub fn get_postgres_types(&self, category: &DataTypeCategory) -> Option<&Vec<PostgresTypes>> {
        self.postgres.get(category)
    }

    pub fn get_mysql_types(&self, category: &DataTypeCategory) -> Option<&Vec<MySqlTypes>> {
        self.mysql.get(category)
    }

    pub fn get_sqlite_types(&self, category: &DataTypeCategory) -> Option<&Vec<SqliteTypes>> {
        self.sqlite.get(category)
    }
}
