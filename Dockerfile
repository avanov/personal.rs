FROM scratch

ADD target/x86_64-unknown-linux-musl/release/myapp /
EXPOSE 3000

CMD ["/myapp"]
