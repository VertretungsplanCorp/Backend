FROM ubuntu:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y wget tar libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libpq-dev software-properties-common && \
    rm -rf /var/lib/apt/lists/* && \
    add-apt-repository ppa:deadsnakes/ppa && \
    apt install -y python3.13-full python3.13-dev python3.13-venv

COPY .env .
COPY server.nu .
COPY vp-backend .
COPY requirements.txt .

ENV LANG en_US.UTF-8
RUN python3.13 -m venv /opt/vp-env
ENV PATH="/opt/vp-env/bin:$PATH"
RUN pip install -r requirements.txt

CMD ["./vp-backend"]
