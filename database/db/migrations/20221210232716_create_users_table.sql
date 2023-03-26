-- migrate:up
create extension if not exists "uuid-ossp";

create table users (
  user_uuid uuid default uuid_generate_v4 () primary key,
  username varchar(255) not null,
  email varchar(255) unique not null,
  password varchar(255) not null
);

-- migrate:down
drop table users;
