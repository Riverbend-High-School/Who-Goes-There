## Build Stage
# Pull base image and update
FROM rust:latest AS builder

RUN update-ca-certificates

# Get Postgres
RUN apt update
RUN apt install -y libpq-dev postgresql npm git

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
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/wgt_backend /app/wgt_backend
COPY --from=builder /app/frontend/wgt-frontend/dist/ /app/static
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

RUN chmod +x /app/entrypoint.sh
RUN chmod +x /app/wgt_backend
RUN apk update
RUN apk add --no-cache postgresql gettext
RUN apk add --no-cache --upgrade bash

RUN ls -R /app

USER $USER:$USER

# Expose web http port
EXPOSE 9999

CMD ["sh", "/app/entrypoint.sh"]