use const_format::concatcp;

use chrono::SecondsFormat::Millis;
use rusqlite::{Connection, OptionalExtension, Result};

use ulid;

use crate::types::{AddMailboxDetails, TMailbox};

const ALL_COLUMNS: &str = "id, local_part, domain, comment, routes_to, created, deleted";
const ALL_PLACEHOLDERS: &str = "?, ?, ?, ?, ?, ?, ?";

fn row_to_mailbox(row: &rusqlite::Row<'_>) -> Result<TMailbox, rusqlite::Error> {
    Ok(TMailbox {
        id: row.get(0)?,
        local_part: row.get(1)?,
        domain: row.get(2)?,
        comment: row.get(3)?,
        routes_to: row.get(4)?,
        created: row.get(5)?,
        deleted: row.get(6)?,
    })
}

const LIST_QUERY: &str = concatcp!("SELECT ", ALL_COLUMNS, " FROM mailboxes");

pub(crate) fn list_mailboxes(db_path: &str) -> Result<Vec<TMailbox>> {
    Connection::open(db_path)
        .map(|conn| {
            conn.prepare(LIST_QUERY)
                .map(|mut st| {
                    st.query_map([], row_to_mailbox)
                        .map(|row_iter| row_iter.collect::<Vec<_>>())
                })
                .flatten()
                .map(|v_r_mailbox| {
                    let mut v_mailbox: Vec<TMailbox> = Vec::new();

                    let first_error = v_r_mailbox.into_iter().fold(None, |acc, item| match acc {
                        Some(_) => acc,
                        None => match item {
                            Err(err) => Some(err),
                            Ok(row) => {
                                v_mailbox.push(row);
                                acc
                            }
                        },
                    });

                    match first_error {
                        Some(e) => Err(e),
                        None => Ok(v_mailbox),
                    }
                })
                .flatten()
        })
        .flatten()
}

const GET_QUERY: &str = concatcp!("SELECT ", ALL_COLUMNS, " FROM mailboxes WHERE id = ?");

pub(crate) fn get_mailbox(db_path: &str, id: &str) -> Result<Option<TMailbox>> {
    Connection::open(db_path)
        .map(|conn| {
            conn.prepare(GET_QUERY)
                .map(|mut st| st.query_one([id], row_to_mailbox).optional())
                .flatten()
        })
        .flatten()
}

const DELETE_QUERY: &str = concatcp!(
    "UPDATE mailboxes SET deleted = COALESCE(deleted, ?) WHERE id = ? RETURNING ",
    ALL_COLUMNS
);

pub(crate) fn delete_mailbox(db_path: &str, id: &str) -> Result<Option<TMailbox>> {
    let now = chrono::Utc::now().to_rfc3339_opts(Millis, true);

    Connection::open(db_path)
        .map(|conn| {
            conn.prepare(DELETE_QUERY)
                .map(|mut st| {
                    st.query_one([now, id.to_owned()], row_to_mailbox)
                        .optional()
                })
                .flatten()
        })
        .flatten()
}

const UNDELETE_QUERY: &str = concatcp!(
    "UPDATE mailboxes SET deleted = NULL WHERE id = ? RETURNING ",
    ALL_COLUMNS
);

pub(crate) fn undelete_mailbox(db_path: &str, id: &str) -> Result<Option<TMailbox>> {
    Connection::open(db_path)
        .map(|conn| {
            conn.prepare(UNDELETE_QUERY)
                .map(|mut st| st.query_one([id.to_owned()], row_to_mailbox).optional())
                .flatten()
        })
        .flatten()
}

const ADD_QUERY: &str = concatcp!(
    "INSERT INTO mailboxes (",
    ALL_COLUMNS,
    ") VALUES (",
    ALL_PLACEHOLDERS,
    ")"
);

pub(crate) fn create_mailbox_from_request(
    db_path: &str,
    request: AddMailboxDetails,
) -> Result<TMailbox> {
    let now = chrono::Utc::now().to_rfc3339_opts(Millis, true);

    let mailbox = TMailbox {
        id: ulid::Ulid::generate().to_string(),
        local_part: request.local_part,
        domain: request.domain,
        comment: request.comment,
        routes_to: request.routes_to,
        created: now,
        deleted: None,
    };

    let values = (
        &mailbox.id,
        &mailbox.local_part,
        &mailbox.domain,
        &mailbox.comment,
        &mailbox.routes_to,
        &mailbox.created,
        None::<String>,
    );

    Connection::open(db_path)
        .map(|conn| {
            conn.prepare(ADD_QUERY)
                .map(|mut st| st.execute(values))
                .flatten()
        })
        .flatten()
        .and(Ok(mailbox))
}
