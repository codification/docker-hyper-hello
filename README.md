# Hello world using docker, rust and hyper

Build and run the server (occupies shell until cancelled by Ctrl-c):

    docker build -t hello-server hello-server && docker run --rm --name hello-server

Build and run the client:

    docker build -t hello-client hello-client

    # Lots of output...

    docker run --rm --link hello-server hello-client
    Response: Hello World!
