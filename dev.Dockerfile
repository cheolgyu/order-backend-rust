FROM localhost:5000/order-rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build
EXPOSE 3000
CMD [ "cargo", "watch","-x","run" ]



