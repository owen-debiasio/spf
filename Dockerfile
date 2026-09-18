FROM archlinux:latest

# Dependencies
RUN pacman -Syu --noconfirm base-devel rust cargo

# Setup source
WORKDIR /usr/src/spf
COPY . .

# Build and copy
RUN cargo build --release
RUN cp target/release/spf /usr/bin/spf

ENV PS1="[spf-dev \W]\$ "

RUN echo "PS1='[spf-dev \W]\$ '" >> /root/.bashrc && \
    echo "echo -e 'Welcome to spf-dev!\n -> Run \"spf\" to use spf.\n'" >> /root/.bashrc

CMD [ "/bin/bash" ]
