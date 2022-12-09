FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/aaron_blog /usr/local/bin/aaron_blog
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["aaron_blog"]
