# badprog.com
FROM ubuntu:24.04

ARG USERNAME=badprogUser
ARG GROUPNAME=badprogGroup
ARG UID=1000
ARG GID=1000

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    ca-certificates \
    tree \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# ubuntu:24.04 ships a default "ubuntu" user/group on UID/GID 1000,
# remove it first to avoid a collision with our own UID/GID
# But only if it exists: "if getent passwd XXX" return 0 if it exists
RUN if getent passwd ${UID} > /dev/null; then userdel -r "$(getent passwd ${UID} | cut -d: -f1)"; fi \
    && if getent group ${GID} > /dev/null; then groupdel "$(getent group ${GID} | cut -d: -f1)"; fi

RUN groupadd -g ${GID} ${GROUPNAME} \
    && useradd -m -u ${UID} -g ${GROUPNAME} -s /bin/bash ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER ${USERNAME}
WORKDIR /home/${USERNAME}

# Set up user environment
RUN echo "alias l='ls -la'" >> /home/$USERNAME/.bashrc

# Install rustup as the non-root user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain 1.96.0 \
    --component clippy \
    --component rustfmt

ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

WORKDIR /workspaces
