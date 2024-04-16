use cid::Cid;
use sqlx::FromRow;
use time::OffsetDateTime;

use crate::database::types::DCid;
use crate::database::DatabaseConnection;

/*
CREATE TABLE root_cids (
    id SERIAL PRIMARY KEY,
    cid VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

#[derive(FromRow, Debug)]
pub struct RootCid {
    id: i64,
    cid: DCid,
    created_at: OffsetDateTime,
}

impl RootCid {
    /*
        pub async fn create(cid: Cid, conn: &DatabaseConnection) -> Result<RootCid, sqlx::Error> {
            let dcid = cid.into();
            let root_cid = sqlx::query_as!(
                RootCid,
                r#"
                INSERT INTO root_cid (cid, created_at)
                VALUES (DEFAULT, $1, CURRENT_TIMESTAMP)
                RETURNING *"#,
                dcid
            )
            .fetch_one(conn)
            .await?;
            Ok(root_cid)
        }
    */
    pub async fn read_most_recent(
        conn: &mut DatabaseConnection,
    ) -> Result<Option<RootCid>, sqlx::Error> {
        let root_cid = sqlx::query_as!(
            RootCid,
            r#"SELECT id, cid as "cid: DCid", created_at FROM root_cids
            ORDER BY created_at DESC LIMIT 1"#
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(root_cid)
    }

    pub fn cid(&self) -> Cid {
        self.cid.into()
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }
}
