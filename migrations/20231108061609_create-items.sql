create table item (
    item_id uuid primary key default gen_random_uuid(),
    content text not null,
    created_at timestamptz not null default now()
);

create index on item(created_at desc);
