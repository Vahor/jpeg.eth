use crate::image::Image;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn get_image(conn: &Connection, nft_id: i32) -> Result<Image, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"SELECT i.image_id, i.url
FROM nft n
JOIN images i ON n.image_id = i.image_id
WHERE n.nft_id = ?;
"#,
    )?;

    let mut rows = stmt.query(&[&nft_id])?;

    let row = rows.next();

    if row.is_err() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let row = row.unwrap();
    if row.is_none() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let row = row.unwrap();

    Ok(Image {
        image_id: row.get(0)?,
        url: row.get(1)?,
    })
}
