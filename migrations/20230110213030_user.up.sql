create table "user" (
  id uuid primary key,
  name text not null,
  email text collate "case_insensitive" unique not null,
  avatar_seed text not null,
  hash text not null,
  is_guest boolean not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz
);

SELECT trigger_updated_at('"user"');