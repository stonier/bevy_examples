################################################################################
# Base
#  - a rust slim-bullseye image
# Installs
#  - convenience tools
#  - adds a user called 'zen'
# Size
#  - 300MB
################################################################################

ARG RUST_VERSION=1.68
ARG DEBIAN_VERSION=bullseye

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION}

ARG NAME=bevy-zen

################################################################################
# Poetry
################################################################################

RUN apt-get update && apt-get install -y --no-install-recommends \
    # For convenience
    bash \
    bash-completion \
    ca-certificates \
    curl \
    git \
    less \
    ssh \
    vim \
    wget \
    # For cargo/rust
    pkg-config \
    # For bevy
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    # X11
    x11-xserver-utils \
    # OpenGL (glxgears + gllibs (libgl#, libglvnd#, libglx#))
    mesa-utils \
    # Vulkan (vulkaninfo + vkcube + vulkanlibs)
    libvulkan1 \
    libvulkan-dev \
    vulkan-tools \
    # EGL
    libegl1 \
    libgles2

################################################################################
# NVIDIA
################################################################################

ENV NVIDIA_VISIBLE_DEVICES ${NVIDIA_VISIBLE_DEVICES:-all}
ENV NVIDIA_DRIVER_CAPABILITIES ${NVIDIA_DRIVER_CAPABILITIES:+$NVIDIA_DRIVER_CAPABILITIES,}graphics,display,video,utility,compute

################################################################################
# Login Shells for Debugging & Development
################################################################################

# In a login shell (below), the PATH env doesn't survive, configure it at ground zero
RUN echo "export PATH=${POETRY_HOME}/bin:${PATH}" >> /etc/profile
ENV TERM xterm-256color
ENTRYPOINT ["/bin/bash", "--login", "-i"]

################################################################################
# Development with a user, e.g. for vscode devcontainers
################################################################################

ARG USERNAME=zen
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -s "/bin/bash" -m $USERNAME && \
    apt-get install -y sudo && \
    echo "${USERNAME} ALL=NOPASSWD: ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}
RUN echo "export PS1='\[\033[01;36m\](docker)\[\033[00m\] \[\033[01;32m\]\u@${NAME}\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> /home/${USERNAME}/.bashrc && \
    echo "alias ll='ls --color=auto -alFNh'" >> /home/${USERNAME}/.bashrc && \
    echo "alias ls='ls --color=auto -Nh'" >> /home/${USERNAME}/.bashrc

#    touch /home/${USERNAME}/.bash_completion && chown ${USERNAME}:${USERNAME} /home/${USERNAME}/.bash_completion

################################################################################
# Debugging with root
################################################################################

RUN echo "export PS1='\[\033[01;36m\](docker)\[\033[00m\] \[\033[01;32m\]\u@${NAME}\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> ${HOME}/.bashrc && \
    echo "alias ll='ls --color=auto -alFNh'" >> ${HOME}/.bashrc && \
    echo "alias ls='ls --color=auto -Nh'" >> ${HOME}/.bashrc
