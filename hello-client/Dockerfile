FROM jimmycuadra/rust

ADD . /my-source

WORKDIR /my-source

RUN ["cargo", "build", "--release"]

CMD ["target/release/hello-client"]

