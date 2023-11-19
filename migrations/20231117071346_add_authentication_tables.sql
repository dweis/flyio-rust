create table users
(
    user_id uuid primary key default gen_random_uuid(),                                                                                                                        
    email text not null unique,
    password text not null,
    created_at timestamptz not null default now() 
);

drop table todo;

create table todos (                                                                                                                                                            
    todo_id uuid primary key default gen_random_uuid(),                                                                                                                        
    content text not null,                                                                                                                                                     
    done boolean not null default false,
    user_id uuid not null references users(user_id),
    created_at timestamptz not null default now()                                                                                                                              
);                                                                                                                                                                             
                                                                                                                                                                               
create index on todos(created_at desc);
create index on todos(user_id)
