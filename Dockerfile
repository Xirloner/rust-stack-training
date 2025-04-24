FROM rust
WORKDIR /usr/src/myapp
COPY . .
# RUN cargo install –path .
EXPOSE 3000
RUN cargo install --path .
CMD ["hello_world"]
