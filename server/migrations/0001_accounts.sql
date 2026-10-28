-- Comptes, départements, droits par département, sessions.

create table accounts (
    id            uuid primary key default gen_random_uuid(),
    -- toujours en minuscules : l'unicité est celle de la colonne
    email         text not null unique check (email = lower(email)),
    name          text not null,
    password_hash text not null,
    -- tous les droits dans tous les départements
    is_admin      boolean not null default false,
    created_at    timestamptz not null default now()
);

create table departments (
    id         uuid primary key default gen_random_uuid(),
    slug       text not null unique check (slug ~ '^[a-z0-9]+(-[a-z0-9]+)*$'),
    name       text not null,
    created_at timestamptz not null default now()
);

-- Un droit est un booléen cumulable : sa présence l'accorde. Le catalogue des droits
-- est dans le code (server/src/rights.rs), pour ne pas migrer à chaque nouveau droit.
create table account_rights (
    account_id    uuid not null references accounts (id) on delete cascade,
    department_id uuid not null references departments (id) on delete cascade,
    right_name    text not null,
    primary key (account_id, department_id, right_name)
);
create index account_rights_department on account_rights (department_id);

-- On ne garde que l'empreinte (sha256) du jeton : une fuite de la base ne donne pas de session.
create table sessions (
    token_hash bytea primary key,
    account_id uuid not null references accounts (id) on delete cascade,
    created_at timestamptz not null default now(),
    expires_at timestamptz not null
);
create index sessions_account on sessions (account_id);
create index sessions_expiry on sessions (expires_at);
