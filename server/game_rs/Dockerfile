# Anirudh Konidala: dockerfile for running rust server microservices!

FROM rust:1.90-bookworm

# install necessary dependencies via. apt
RUN apt update && apt install -y \
   python3 \
   python3-pip \
   python3-venv \
   curl

# use uv because its faster and written in rust lol lmao (rust is just better lmao)
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:${PATH}"
RUN uv tool install maturin

WORKDIR /app

COPY . .

RUN uv venv /venv
ENV VIRTUAL_ENV="/venv"
ENV PATH="/venv/bin:${PATH}"
RUN maturin develop

CMD ["python3", "test_game.py"]
