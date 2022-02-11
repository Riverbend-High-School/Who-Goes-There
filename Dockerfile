## Build Stage
# Pull base image and update
FROM rust:latest AS builder

RUN update-ca-certificates

# Get Postgres
RUN apt update
RUN apt install -y libpq-dev postgresql
RUN apt install -y npm
RUN apt install -y git

# Create app user
ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN git clone https://github.com/Riverbend-High-School/wgt-backend.git /app

RUN chown -R "${USER}":"${USER}" /app

# Move to repo
WORKDIR /app

# Build app
RUN cargo build --release

# Build frontend
WORKDIR /app/frontend/wgt-frontend

RUN npm i
RUN npm run build

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/wgt_backend /app/wgt_backend

RUN apt update -y
RUN apt install -y postgresql-11

USER $USER:$USER

# Expose web http port
EXPOSE 9999

ENTRYPOINT ["/app/entrypoint.sh"]