-- migrate:up
create type TaskState as enum('NotStarted','InProgress','Completed');

create table tasks (
  task_uuid uuid primary key,
  state TaskState,
  title varchar(255),
  description varchar(255),
  user_uuid uuid references users (user_uuid) on delete cascade
);

-- migrate:down
drop table tasks;
