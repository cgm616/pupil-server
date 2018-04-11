create table conf (
  id serial primary key,
  created timestamptz not null,
  userid int references users(id),
  username varchar not null unique,
  link varchar not null unique
)