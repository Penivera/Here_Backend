
use sea_orm::{
    col_err, sea_query::Type, ColumnType, DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use postgis; // Make sure this is imported

// 1. Define our newtype wrapper
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PgPoint(pub postgis::Point);

// 2. Implement From<postgis::Point> for PgPoint
impl From<postgis::Point> for PgPoint {
    fn from(point: postgis::Point) -> Self {
        PgPoint(point)
    }
}

// 3. Implement From<PgPoint> for Value
// This tells SeaORM how to convert our Rust type INTO a database value
impl From<PgPoint> for Value {
    fn from(pg_point: PgPoint) -> Self {
        // We use the `postgis::Point` as the underlying value
        Value::from(pg_point.0)
    }
}

// 4. Implement TryGetable for PgPoint
// This tells SeaORM how to convert a database value BACK into our Rust type
impl TryGetable for PgPoint {
    fn try_get_by(res: &QueryResult, index: &str) -> Result<Self, TryGetError> {
        let point: postgis::Point = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        Ok(PgPoint(point))
    }

    fn try_get_by_index(res: &QueryResult, index: usize) -> Result<Self, TryGetError> {
        let point: postgis::Point = res.try_get_by_index(index).map_err(TryGetError::DbErr)?;
        Ok(PgPoint(point))
    }
}

// 5. Implement ColumnType for PgPoint
// This defines the database column type
impl ColumnType for PgPoint {
    fn def(&self) -> sea_orm::ColumnDef {
        // Using "geometry(Point, 4326)"
        // 4326 is the standard SRID for WGS 84 (GPS coordinates)
        sea_orm::ColumnDef::new(Type::Custom("geometry(Point, 4326)".into()))
    }
}

// 6. Implement From<PgPoint> for postgis::Point (helper)
impl From<PgPoint> for postgis::Point {
    fn from(pg_point: PgPoint) -> Self {
        pg_point.0
    }
}