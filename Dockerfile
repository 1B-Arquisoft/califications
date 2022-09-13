FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/califications

COPY . .
# RUN curl github.com

RUN cargo install --path .


CMD [ "califications"]



