FROM archlinux/base

# install restic
RUN pacman -Syu --noconfirm restic

# copy over the entry script, help files
COPY ./main.sh /main.sh
COPY ./mcrcon /mcrcon

#run
CMD ./main.sh
