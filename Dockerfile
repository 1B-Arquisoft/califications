FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/califications

COPY . .
# RUN curl github.com

# RUN cargo install --path .

RUN cargo build


# CMD [ "califications"]
CMD [ "cargo", "run" ]



