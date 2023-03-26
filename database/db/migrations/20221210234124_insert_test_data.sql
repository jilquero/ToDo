-- migrate:up
insert into users
values ('4c28bc86-52d5-4a15-979b-76af54ec4862', 'adminn', 'admin@admin.com', '$argon2i$v=19$m=4096,t=3,p=1$c3VwZXJzZWN1cmVzYWx0$aDqIqVIi7KLWgAr3Ec7mDsMUlI2rocvobYQeS6GmZY4'),
       ('5d0b91ff-800b-44be-a20d-12c4021a39f4', 'test', 'test@test.com', 'test'),
       ('5457b292-65f9-4afd-a8e0-4b7deb0b9fe2', 'sth', 'sth@sth.com', 'sth'),
       ('5457b292-65f9-4afd-a8e0-4b7deb0b9fe1', 'lelelel', 'lel@lel.com', '$argon2i$v=19$m=4096,t=3,p=1$c3VwZXJzZWN1cmVzYWx0$oN6nnKhecacVszGCnOyiIipzUntJu+8AFlt2iHwK13Q');

insert into tasks
values ('59c202ba-4732-46e5-8011-1b3f2d68d541', 'NotStarted', 'start', 'lol', '4c28bc86-52d5-4a15-979b-76af54ec4862'),
       ('14813c54-c898-4ba3-a417-4ef239ccf805', 'InProgress', 'test', 'aha', '4c28bc86-52d5-4a15-979b-76af54ec4862'),
       ('d922d41c-2ca9-4169-9dd1-69d3c0f2ffb2', 'Completed', 'sth', 'oof', '5d0b91ff-800b-44be-a20d-12c4021a39f4'),
       ('f26471fc-7ede-4d66-8d54-0632678f4ce8', 'NotStarted', 'title', 'description', '5457b292-65f9-4afd-a8e0-4b7deb0b9fe1');