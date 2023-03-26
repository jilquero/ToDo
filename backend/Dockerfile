FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/app
COPY ./backend ./backend
COPY ./common ./common

RUN cargo install --path ./backend

FROM gcr.io/distroless/cc-debian11
COPY --from=build /usr/local/cargo/bin/backend /usr/local/bin/backend
CMD [ "backend" ]
