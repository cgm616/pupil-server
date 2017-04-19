create table users (
  id serial primary key,
  name varchar not null,
  email varchar not null unique,
  username varchar not null unique,
  pass varchar not null,
  conf boolean not null default 'f'
)
