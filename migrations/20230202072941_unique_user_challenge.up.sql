alter table "user_challenge" add constraint one_user_per_challenge unique (user_id, challenge_id);