FROM python:3.6.10-slim-stretch

COPY requirements.txt /tmp/requirements.txt
RUN python3 -m venv /root/.venv \
    && . /root/.venv \
    && pip install -r /tmp/requirements.txt

COPY rakn /usr/bin/rakn
RUN chmod +x /usr/bin/rakn

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
