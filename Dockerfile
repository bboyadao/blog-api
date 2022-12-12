#FROM rustlang/rust:nightly as builder
FROM rust:1.65.0 as builder
RUN apt-get update -y
RUN apt-get install -y libssl1.1
WORKDIR /app
COPY . .
#RUN cargo install --path .
RUN cargo build --release


#FROM debian:buster-slim as runner
FROM alpine as runner
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release .
RUN ls -la
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["aaron_blog"]
