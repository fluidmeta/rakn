FROM debian:stretch-20200130-slim

RUN apt-get update \
    && apt-get install -y git

COPY rakn /usr/bin/rakn
RUN chmod +x /usr/bin/rakn

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
