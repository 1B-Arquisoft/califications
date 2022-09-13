FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/califications

COPY . .


RUN cargo install --path .


CMD [ "califications"]



