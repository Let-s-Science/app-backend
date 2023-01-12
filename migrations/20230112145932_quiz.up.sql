create extension if not exists "uuid-ossp"; 

create table "translation" (
    id uuid primary key default uuid_generate_v1mc(),
    language_code TEXT not null,
    content text not null,
    constraint unique_content
        unique (id, language_code)
);

create table "quiz" (
    id uuid primary key default uuid_generate_v1mc(),
    title uuid not null,
    created_at timestamptz not null default now(),
    created_by uuid not null,
    constraint fk_created_by
        foreign key(created_by)
            references "user"(id),
    constraint fk_title
        foreign key(title)
            references "translation"(id)
);

create table "question" (
    id uuid primary key default uuid_generate_v1mc(),
    quiz_id uuid not null,
    question uuid not null,
    data json not null,
    constraint fk_quiz_id
        foreign key(quiz_id)
            references "quiz"(id)
);
