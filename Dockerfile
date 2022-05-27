## Build Stage
# Pull base image and update
FROM rust:slim-buster as builder

USER root

RUN update-ca-certificates


ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Get Postgres
RUN apt update
RUN apt install -y libpq-dev postgresql nodejs npm git
RUN apt update
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
FROM debian:buster-slim

ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

RUN apt update \
    && apt install tzdata 

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/wgt_backend /app/wgt_backend
COPY --from=builder /app/frontend/wgt-frontend/dist/ /app/static
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

RUN chown -R "${USER}":"${USER}" /app

RUN chmod +x /app/entrypoint.sh
RUN apt install libpq-dev gettext

USER $USER:$USER

# Expose web http port
EXPOSE 9999

ENTRYPOINT ["sh", "/app/entrypoint.sh"]