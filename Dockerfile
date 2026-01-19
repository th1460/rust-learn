FROM ubuntu
WORKDIR workspace
RUN apt update && apt install -y curl build-essential neovim tmux
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENTRYPOINT tmux
