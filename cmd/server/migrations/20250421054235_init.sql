-- Add migration script here
create table users (
    id uuid primary key default gen_random_uuid(),
    email text not null unique,
    password text not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create table if not exists todos (
    id serial primary key,
    title text not null,
    completed boolean not null default false
);
