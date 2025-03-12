FROM --platform=linux/amd64 debian:latest

ARG USER_NAME
ARG ROOT_PASS
ARG USER_PASS

RUN apt-get update                 \
    && apt-get upgrade -y          \
    && apt-get install -y sudo

COPY "init/scripts/install/*.sh" "/root/.install/"
COPY "init/scripts/init.sh"      "/home/.init.sh"
COPY "init/dotfiles/.bashrc"     "/home/${USER_NAME}/.bashrc"

RUN sh "/root/.install/default-tools.sh"
RUN sh "/root/.install/dev-tools.sh"

RUN useradd -m -s "/bin/bash" "${USER_NAME}"
RUN usermod -aG sudo "${USER_NAME}"

RUN echo "root:${ROOT_PASS}" | chpasswd
RUN echo "${USER_NAME}:${USER_PASS}" | chpasswd

RUN chown -R "${USER_NAME}:${USER_NAME}" "/home"

USER ${USER_NAME}
WORKDIR /home/${USER_NAME}/rkfs

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN . "$HOME/.cargo/env"

ENTRYPOINT [ "sh", "/home/.init.sh" ]
