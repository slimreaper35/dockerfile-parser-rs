FROM ubuntu:22.04 AS builder

LABEL maintainer="example@example.com"

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
      apt-get install -y --no-install-recommends \
      build-essential \
      curl \
      git \
      ca-certificates

WORKDIR /app

COPY . .

RUN make build

FROM alpine:3.18

LABEL description="This is a minimal runtime image"

RUN apk add --no-cache \
      ca-certificates \
      curl

WORKDIR /app

COPY --from=builder /app/mybinary .

EXPOSE 8080

ENV ENVIRONMENT=production

ENTRYPOINT ["./mybinary"]

CMD ["--help"]
