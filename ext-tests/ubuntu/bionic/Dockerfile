FROM ubuntu:bionic-20200112

RUN apt-get update \
    && apt-get install -y git=1:2.17.1-1ubuntu0.5

COPY rakn /usr/bin/rakn
RUN chmod +x /usr/bin/rakn

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
