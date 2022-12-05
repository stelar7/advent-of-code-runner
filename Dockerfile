# 1. Start with an Ubuntu base image
FROM ubuntu:22.04

# 2. Configure TZ, so we don't get interactive prompt
ENV TZ=Europe/Oslo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# 3. Build things from source first, so get build deps installed
RUN apt-get update && apt-get install -yqq --no-install-recommends \
  build-essential git wget ca-certificates curl unzip ninja-build ccache gnupg apt-transport-https gpg

# 4. Tell apt to install node.js from nodesource.com, to get v16.x instead of v12.x
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -

# 5. Install cmake
RUN apt remove --purge --auto-remove cmake; \
  apt update; \
  apt install -y software-properties-common lsb-release; \
  apt clean all; \
  wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | tee /usr/share/keyrings/kitware-archive-keyring.gpg >/dev/null; \
  echo 'deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ jammy main' | tee /etc/apt/sources.list.d/kitware.list >/dev/null; \
  apt update; \
  apt install -yqq cmake kitware-archive-keyring;

# 6. Install Mono and .NET (C#)
RUN apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys 3FA7E0328081BFF6A14DA29AA6A19B38D3D831EF; \
  echo "deb https://download.mono-project.com/repo/ubuntu stable-focal main" | tee /etc/apt/sources.list.d/mono-official-stable.list; \
  wget https://packages.microsoft.com/config/ubuntu/20.04/packages-microsoft-prod.deb -O packages-microsoft-prod.deb; \
  dpkg -i packages-microsoft-prod.deb; \
  rm packages-microsoft-prod.deb; \
  apt update; \
  apt install -yqq mono-complete dotnet-sdk-6.0; 

# 7. Install all other compilers, from apt-get
RUN apt-get update && apt-get install -yqq --no-install-recommends\
  cargo elixir erlang openjdk-17-jdk golang nodejs php-cli python3 ruby rustc \
  && rm -rf /var/lib/apt/lists/* \
  && apt-get autoremove -yqq

# 8. Install 05AB1E
RUN git clone https://github.com/Adriandmen/05AB1E.git && \
  cd 05AB1E && \
  (yes | PATH=/usr/bin:$PATH mix local.hex --force) && \
  PATH=/usr/bin:$PATH mix deps.get && \
  (yes | PATH=/usr/bin:$PATH MIX_ENV=prod mix escript.build) && \ 
  mv osabie /usr/local/bin/osabie && \
  cd .. && rm -rf 05AB1E

# 9. Install Golfscript
RUN git clone https://github.com/darrenks/golfscript.git && \
  cd golfscript && \
  chmod +x golfscript.rb && \
  mv golfscript.rb /usr/local/bin/golfscript && \
  cd .. && rm -rf golfscript

# 9. Create test folder
RUN mkdir test;