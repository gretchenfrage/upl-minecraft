FROM archlinux/base

COPY ./downproxy /downproxy

ENV RUST_LOG==trace

RUN pacman -Syu --noconfirm openssl

EXPOSE 25565:25565

CMD /downproxy run_server 25565
