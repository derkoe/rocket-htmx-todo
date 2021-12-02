FROM rust:slim as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
EXPOSE 8000
COPY --from=build-env /app/target/release/rocket-htmx-todo /
CMD ["./rocket-htmx-todo"]
