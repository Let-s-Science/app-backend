create table "translation" (
    id uuid primary key,
    content text not null
);

create table "quiz" (
    id uuid primary key,
    title uuid not null,
    created_at timestamptz not null default now(),
    created_by uuid not null,
    constraint fk_created_by
        foreign key(created_by)
            references "user"(id),
    constraint fk_title
        foreign key(title)
            references "translation"(id)
)