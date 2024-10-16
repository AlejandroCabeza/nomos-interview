use crate::errors::{AffectedRows, PersistenceError};
use sqlx::{Pool, Sqlite};

pub async fn set_score(score: u16, pool: &Pool<Sqlite>) -> Result<(), PersistenceError> {
    // TODO: Generalise it
    let rows_affected = sqlx::query!(
                    r#"
                        UPDATE games
                        SET score = ?1
                        WHERE name = 'Who Is That Pokemon'
                    "#,
                    score
                )
        .execute(pool)
        .await
        .map_err(PersistenceError::Sqlx)?
        .rows_affected();

    if rows_affected == 1 {
        Ok(())
    } else {
        let affected_rows = AffectedRows::new(rows_affected as usize, 1);
        Err(PersistenceError::UnexpectedAffectedRows(affected_rows))
    }
}
