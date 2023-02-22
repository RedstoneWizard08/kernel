FROM ubuntu:22.04

ARG GNU_A_RELEASE=10.3-2021.07

ARG GCC_AARCH64="https://developer.arm.com/-/media/Files/downloads/gnu-a/${GNU_A_RELEASE}/binrel/gcc-arm-${GNU_A_RELEASE}-aarch64-aarch64-none-elf.tar.xz"
ARG GCC_X86_64="https://developer.arm.com/-/media/Files/downloads/gnu-a/${GNU_A_RELEASE}/binrel/gcc-arm-${GNU_A_RELEASE}-x86_64-aarch64-none-elf.tar.xz"

# Ruby gems
COPY Gemfile .

# Libraries
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install automake bison \
    build-essential flex git libtool \
    ninja-build pkg-config wget \
    ca-certificates gdb-multiarch \
    libpixman-1-dev libglib2.0-dev \
    libusb-1.0.0-dev locales python3 \
    ruby ruby-dev bash curl

# GCC aarch64 tools
RUN if [ "$(uname -m)" = "aarch64" ]; then wget ${GCC_AARCH64}; else wget ${GCC_AARCH64}; fi
RUN tar -xf gcc-arm-${GNU_A_RELEASE}*
RUN cp \
    gcc-arm-${GNU_A_RELEASE}*/bin/aarch64-none-elf-objdump \
    gcc-arm-${GNU_A_RELEASE}*/bin/aarch64-none-elf-readelf \
    gcc-arm-${GNU_A_RELEASE}*/bin/aarch64-none-elf-nm \
    /usr/local/bin/

# Ruby dependencies
RUN gem install bundler
RUN bundle config set --local without "development" && bundle install --retry 3

# QEMU
RUN git clone https://gitlab.com/qemu-project/qemu.git /usr/src/qemu
WORKDIR /usr/src/qemu
RUN git checkout tags/v6.1.0
RUN ./configure --target-list=aarch64-softmmu --enable-modules \
    --enable-tcg-interpreter --enable-debug-tcg --python=/usr/bin/python3
RUN make -j$(nproc)
RUN make install
WORKDIR /usr/src
RUN rm -rf qemu

# OpenOCD
RUN git clone --depth 1 https://git.code.sf.net/p/openocd/code /usr/src/openocd
WORKDIR /usr/src/openocd
RUN ./bootstrap
RUN ./configure --enable-ftdi
RUN make -j$(nproc)
RUN make install

# GDB
RUN wget -P ~ git.io/.gdbinit

# Rustup
RUN wget https://sh.rustup.rs -O rustup.sh
RUN bash rustup.sh --default-toolchain nightly --profile complete --target aarch64-unknown-none-softfloat --component rust-src -y
RUN rm rustup.sh

# Cleanup
RUN apt-get -y autoremove
RUN apt-get -y clean
RUN rm -rf /var/lib/apt/lists/*

# Locales
RUN locale-gen en_US.UTF-8

ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8 \
    RUBYOPT=-W0

# Openocd
COPY rpi3.cfg /usr/src/openocd/
COPY rpi4.cfg /usr/src/openocd/

# GDB
COPY auto /root/.gdbinit.d/auto

# Finalize
SHELL [ "/bin/bash", "-c" ]
CMD [ "/bin/bash" ]
