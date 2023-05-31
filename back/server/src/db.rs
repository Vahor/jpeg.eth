use log::warn;
use crate::image::Image;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn get_image(conn: &Connection, token_id: &str) -> Result<Image, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
SELECT n.image_id, n.attributes
FROM nft n
WHERE n.token_id = ?;
"#,
    )?;

    let mut rows = stmt.query(&[&token_id])?;

    let row = rows.next();

    if row.is_err() {
        warn!("Error while querying image for token {}", token_id);
        return Err(rusqlite::Error::InvalidQuery);
    }

    let row = row.unwrap();
    if row.is_none() {
        warn!("No image found for token {}", token_id);
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let row = row.unwrap();

    let raw_attributes = row.get::<usize, String>(1).unwrap();
    let attributes = serde_json::from_str(&*raw_attributes).unwrap();

    Ok(Image {
        image_id: row.get(0)?,
        attributes,
    })
}

pub fn get_all_images(conn: &Connection) -> Result<Vec<Image>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
SELECT n.image_id, n.attributes
FROM nft n
WHERE n.token_id IS NOT NULL;
"#,
    )?;

    let mut rows = stmt.query([])?;

    let mut images = Vec::new();

    while let Some(row) = rows.next()? {
        let image_id = row.get::<usize, String>(0).unwrap();
        let raw_attributes = row.get::<usize, String>(1).unwrap();
        let attributes = serde_json::from_str(&*raw_attributes).unwrap();

        images.push(Image {
            image_id,
            attributes,
        });
    }

    Ok(images)
}

pub fn register_image(
    conn: &Connection,
    image_id: String,
    attributes: String,
) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
INSERT INTO nft (image_id, attributes)
VALUES (?, ?);
"#,
    )?;

    let result = stmt.execute(&[&image_id, &attributes]);

    if result.is_err() {
        let err = result.unwrap_err();
        // if unique constraint failed, it means the image is already assigned
        if let rusqlite::Error::SqliteFailure(err, _) = err {
            if err.code == rusqlite::ErrorCode::ConstraintViolation {
                return Ok(());
            }
        }

        return Err(err);
    }

    println!("Registered image {}", image_id);

    Ok(())
}

pub fn assign_image(
    conn: &Connection,
    token_id: String,
    image_id: String,
) -> rusqlite::Result<usize> {
    let mut stmt = conn.prepare(
        r#"
UPDATE nft
SET token_id = ?
WHERE image_id = ?;
"#,
    )?;

    return stmt.execute(&[&token_id, &image_id]);
}

pub fn get_random_unassigned_image(conn: &Connection) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare(
        r#"
SELECT n.image_id
FROM nft n
WHERE n.token_id IS NULL
ORDER BY RANDOM()
LIMIT 1;
"#,
    )?;

    let mut rows = stmt.query([])?;

    let row = rows.next();

    if row.is_err() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let row = row.unwrap();
    if row.is_none() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let row = row.unwrap();

    Ok(row.get(0)?)
}

