## Build Stage
# Pull base image and update
FROM ekidd/rust-musl-builder:stable AS builder

USER root

RUN update-ca-certificates


ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Get Postgres
RUN apt update
RUN apt install -y libpq-dev postgresql nodejs npm git
RUN npm install -g n
RUN n stable
RUN npm install -g npm

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

# RUN git clone https://github.com/Riverbend-High-School/wgt-backend.git /app
COPY . /app

RUN chown -R "${USER}":"${USER}" /app

# Move to repo
WORKDIR /app

# Build app
RUN cargo build --release --target x86_64-unknown-linux-musl

# Build frontend

FROM node:lts-alpine as frontend

COPY --from=builder /app/frontend/wgt-frontend /app/frontend

WORKDIR /app/frontend

RUN npm i
RUN npm run build

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/wgt_backend /app/wgt_backend
COPY --from=frontend /app/frontend/dist/ /app/static
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

RUN chown -R "${USER}":"${USER}" /app

RUN chmod +x /app/entrypoint.sh
RUN apk add --no-cache libpq-dev gettext
RUN apk add --no-cache --upgrade bash
RUN rm -rf /var/cache/apk/*

USER $USER:$USER

# Expose web http port
EXPOSE 9999

ENTRYPOINT ["sh", "/app/entrypoint.sh"]