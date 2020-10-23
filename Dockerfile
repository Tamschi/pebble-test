FROM ubuntu:20.04
COPY pebble-sdk-4.5-linux64.tar.bz2 /root/Downloads/pebble-sdk-4.5-linux64.tar.bz2
COPY sdk-core-4.3.tar.bz2 /root/Downloads/sdk-core-4.3.tar.bz2

# Prerequisites
RUN apt-get update
RUN apt-get install -y \
	# for this Dockerfile
	curl tree \
	# Pebble
	gcc python2-dev \
	# Rust
	lld \
	# Pebble emulator
	libsdl1.2debian libfdt1 libpixman-1-0

# Also Pebble.
# This one pulls A TON of dependencies, and one of them is tzdata which is interactive by default.
RUN  DEBIAN_FRONTEND=noninteractive apt-get install -y npm

RUN echo '#! /bin/bash' > /usr/bin/python
RUN echo 'python2 "$@"' >> /usr/bin/python
RUN cat /usr/bin/python
RUN chmod +x /usr/bin/python

# For things like "You must have npm ≥ 3.0.0 available on your path."
RUN echo 'export PYTHONIOENCODING=utf8' >> ~/.bashrc_non_interactive

# Docker runs code in non-interactive shells, which won't execute ~/.bachrc!
# However, this actually doesn't work...
ENV BASH_ENV="$HOME/.bashrc_non_interactive"

# Download and install the Pebble SDK
#RUN curl --create-dirs -o ~/Downloads/pebble-sdk-4.5-linux64.tar.bz2 https://developer.rebble.io/s3.amazonaws.com/assets.getpebble.com/pebble-tool/pebble-sdk-4.5-linux64.tar.bz2
RUN mkdir ~/pebble-dev/
RUN ls ~/Downloads
RUN cd ~/pebble-dev && tar -jxf ~/Downloads/pebble-sdk-4.5-linux64.tar.bz2
RUN echo 'export PATH="$PATH:$HOME/pebble-dev/pebble-sdk-4.5-linux64/bin"' >> ~/.bashrc_non_interactive
RUN mkdir ~/.pebble-sdk/
RUN touch ~/.pebble-sdk/NO_TRACKING

# Download and install Python libraries
RUN curl https://bootstrap.pypa.io/2.6/get-pip.py | python2 -
RUN pip install virtualenv
# --no-site-packages isn't supported anymore, but is the default.
RUN cd ~/pebble-dev/pebble-sdk-4.5-linux64 && bash -c "virtualenv .env; source .env/bin/activate; pip install -r requirements.txt; deactivate"

# Install SDK
#RUN . ~/.bashrc_non_interactive; pebble sdk install https://github.com/aveao/PebbleArchive/raw/master/SDKCores/sdk-core-4.3.tar.bz2
RUN . ~/.bashrc_non_interactive; pebble sdk install /root/Downloads/sdk-core-4.3.tar.bz2

# Install emulator? This takes very long.
#RUN . ~/.bashrc_non_interactive; pebble install --emulator basalt

# Install and set up Rust toolchain
# nightly is needed to build a custom std
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
RUN echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc_non_interactive
RUN cat ~/.bashrc_non_interactive
RUN echo $PATH
# I have no idea why this doesn't work otherwise
RUN . ~/.bashrc_non_interactive; rustup component add rust-src
RUN . ~/.bashrc_non_interactive; cargo install xargo

# Build it!
COPY build.sh /root/build.sh
RUN chmod +x ~/build.sh

# Mount your workspace to /mnt/workspace.
# Set $PACKAGE to the package name you want to build.
CMD ["sh", "-v", "/root/build.sh"]
