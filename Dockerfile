FROM rust:1.71.0 as builder
WORKDIR /usr/src/file_upload
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN mkdir -p /opt/file_upload
COPY --from=builder /usr/local/cargo/bin/file_upload /opt/file_upload/
COPY --from=builder /usr/src/file_upload/Rocket.toml /opt/file_upload/
COPY --from=builder /usr/src/file_upload/templates/ /opt/file_upload/templates/
EXPOSE 8000
WORKDIR /opt/file_upload/
ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["./file_upload"]
