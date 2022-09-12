FROM rustlang/rust:nightly

# WORKDIR .

# COPY calificationsms .

RUN cargo install --path .


CMD [ ".califications"]



