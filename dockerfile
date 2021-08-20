# FROM rust as builder

# WORKDIR /app

# COPY . .

# RUN cargo build --release --target x86_64-unknown-linux-musl

# FROM alpine

# COPY --from=builder /app/target/release/expnamination_runner .

# ENTRYPOINT [ "expnamination_runner" ]

FROM alpine

ADD target/x86_64-unknown-linux-musl/release/expnamination_runner /
EXPOSE 3000

CMD ["/expnamination_runner"]