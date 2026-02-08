FROM python:3.9-slim as build

RUN apt-get update && \
    apt-get install -y curl build-essential python3-dev python3-pip && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env && \
    pip install maturin

ENV PATH="/root/.cargo/bin:$PATH"
ENV PYTHON_SYS_EXECUTABLE=/usr/local/bin/python3
ENV PYO3_PYTHON=/usr/local/bin/python3

WORKDIR /app
COPY . .

RUN . $HOME/.cargo/env && cd server/game_rs && maturin build --release --strip --interpreter /usr/local/bin/python3

FROM python:3.9-slim

WORKDIR /app

COPY --from=build /app/server/game_rs/target/wheels/*.whl /app/
COPY . .

RUN pip install --no-cache-dir /app/*.whl

ENV PYTHON_SYS_EXECUTABLE=/usr/local/bin/python3
ENV PYO3_PYTHON=/usr/local/bin/python3

CMD ["python3"]