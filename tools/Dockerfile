FROM ubuntu:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y wget tar libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libpq-dev software-properties-common && \
    rm -rf /var/lib/apt/lists/* && \
    add-apt-repository ppa:deadsnakes/ppa && \
    apt install -y python3.13-full python3.13-dev python3.13-venv

RUN curl -fsSL https://apt.fury.io/nushell/gpg.key | gpg --dearmor -o /etc/apt/trusted.gpg.d/fury-nushell.gpg
RUN echo "deb https://apt.fury.io/nushell/ /" | tee /etc/apt/sources.list.d/fury.list
RUN apt update && apt install nushell

COPY .env .
COPY server.nu .
COPY vp-backend .
COPY requirements.txt .

RUN python3.13 -m venv vp-env
ENV PATH="vp-env/bin:$PATH"
RUN pip install -r requirements.txt

CMD ["nu", "server.nu"]
