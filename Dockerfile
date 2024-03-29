FROM rust:1.63 as builder
RUN cargo install sqlx-cli
WORKDIR /accounting_backend
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bullseye-slim
ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 8000
ENV APP_USER=accounting
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}/db
COPY --from=builder /accounting_backend/target/release/accounting-backend ${APP}/accounting-backend
COPY --from=builder /accounting_backend/db ${APP}/db/
COPY Rocket.toml ${APP}/
RUN chown -R $APP_USER:$APP_USER ${APP}
WORKDIR ${APP}
CMD ["./accounting-backend"]
