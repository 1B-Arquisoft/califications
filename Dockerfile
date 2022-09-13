FROM rustlang/rust:nightly

WORKDIR /usr/src/califications

COPY . .

RUN cargo install --path .


CMD [ "califications"]



