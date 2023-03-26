FROM amacneil/dbmate:v1.12.0

WORKDIR /usr/src/app
COPY ./database/startup.sh .
COPY ./database/db ./db

ENTRYPOINT [ "sh", "./startup.sh" ]