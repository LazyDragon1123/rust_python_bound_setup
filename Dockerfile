FROM python:3.9

ENV POETRY_VERSION=1.3.2

ADD src/ src/

WORKDIR /src

RUN apt-get update && \
    apt-get install -y && \
    curl -sSL https://install.python-poetry.org | POETRY_VERSION=$POETRY_VERSION python3 -

ENV PATH $PATH:/root/.local/bin

COPY poetry.lock pyproject.toml ./

RUN poetry config virtualenvs.create false && \
    poetry install && \
    python -m pip install --upgrade pip

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="$HOME/.cargo/bin:$PATH"

COPY Cargo.lock Cargo.toml ./
