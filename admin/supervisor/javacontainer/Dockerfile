FROM archlinux/base

# install java
RUN                            \
    pacman -Syu --noconfirm    \ 
        jdk8-openjdk           \
        jre8-openjdk           \
        jre8-openjdk-headless
ENV PATH="/usr/lib/jvm/java-8-openjdk/jre/bin/java:$PATH"

# run whatever command is given
CMD eval $COMMAND
