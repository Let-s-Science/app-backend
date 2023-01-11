create table "user" (
  id uuid primary key,
  name text not null,
  email text collate "case_insensitive" unique,
  avatar_seed text not null,
  hash text,
  is_guest boolean not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz,
  score int not null default 0
);

select trigger_updated_at('"user"');

alter table "user" add constraint require_password check
(
  (is_guest and email is null and hash is null) or
  (not is_guest and email is not null and hash is not null)
);