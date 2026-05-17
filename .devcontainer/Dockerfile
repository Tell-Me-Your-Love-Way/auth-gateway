FROM ubuntu:24.04

WORKDIR /home/

COPY . .

RUN useradd -m -s /bin/zsh nocturne

RUN usermod -aG sudo nocturne
RUN usermod -aG adm nocturne
RUN usermod -aG wheel nocturne

RUN bash ./setup.sh

USER nocturne
ENV PATH="/root/.cargo/bin:$PATH"
