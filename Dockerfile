FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/califications

COPY . .
# RUN curl github.com

# RUN cargo install --path .

# RUN cargo --config net.git-fetch-with-cli=true fetch


CMD ["cargo","run"]



