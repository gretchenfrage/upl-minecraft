FROM archlinux/base

COPY ./downproxy /downproxy

ENV RUST_LOG=info,downproxy=trace

EXPOSE 25565:25565

CMD /downproxy run_server 25565
