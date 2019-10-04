FROM localhost:5000/order-rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build
EXPOSE 3000
RUN diesel migration run
CMD [ "cargo", "watch","-x","run" ]
