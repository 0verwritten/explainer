# run this command beforehand: 
# docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release

FROM alpine

ADD target/x86_64-unknown-linux-musl/release/expnamination_runner /
EXPOSE 3000

CMD ["/expnamination_runner"]