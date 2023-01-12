create type challengetype as enum ('dailychallenge', 'counter');

create table "challenge" (
    id uuid primary key default uuid_generate_v1mc(),
    type challengetype not null,
    goal int not null,
    description uuid not null,
    constraint fk_description
        foreign key(description)
            references "translation"(id)
);

create table "user_challenge" (
    user_id uuid not null,
    challenge_id uuid not null,
    progress int not null,
    updated_at timestamptz
);

select trigger_updated_at('"user_challenge"');
