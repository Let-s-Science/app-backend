-- https://docs.rs/poem-dbsession/0.3.51/poem_dbsession/sqlx/struct.PgSessionStorage.html
create table if not exists poem_sessions (
    id varchar not null primary key,
    expires timestamp with time zone null,
    session jsonb not null
);

create index if not exists poem_sessions_expires_idx on poem_sessions (expires);
