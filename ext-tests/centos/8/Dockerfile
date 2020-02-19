FROM centos:centos8.1.1911

RUN yum install -y git

COPY rakn /usr/bin/rakn
RUN chmod +x /usr/bin/rakn

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
