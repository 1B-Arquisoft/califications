FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/califications

COPY . .
# RUN curl github.com

# RUN cargo install --path .

# RUN cargo build

EXPOSE 8000

# CMD [ "califications"]
CMD [ "cargo", "run" ]



