FROM rust:1.77

WORKDIR /usr/src/htmx-warp

COPY . .

RUN cargo install --path .

EXPOSE 3030

CMD ["htmx-warp"]
