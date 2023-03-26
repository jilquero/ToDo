FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/app
COPY ./frontend ./frontend
COPY ./common ./common

RUN apt-get update -y && apt-get install -y npm
RUN wasm-pack -v || cargo install wasm-pack
RUN npm i --prefix ./frontend
RUN npm run build --prefix ./frontend

FROM nginx
COPY --from=build /usr/src/app/frontend/dist /usr/share/nginx/html
CMD [ "nginx", "-g", "daemon off;" ]
