FROM jimmycuadra/rust

EXPOSE 3000

ADD . /my-source

WORKDIR /my-source

RUN ["cargo", "build", "--release"]

CMD ["target/release/hello-server"]

