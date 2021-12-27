FROM ubuntu:20.04

RUN apt-get -y update && apt-get -y install curl && \
        curl --proto '=https' --tlsv1.2 -sSf https://tiup-mirrors.pingcap.com/install.sh | sh && \
        ln -s /root/.tiup/bin/tiup /bin/tiup

ENV TIDB_VER=v5.3.0
ENV PLAYGROUND_VER=v1.8.1

RUN tiup install \
        playground:${PLAYGROUND_VER} \
        pd:${TIDB_VER} \
        tikv:${TIDB_VER} \
        tidb:${TIDB_VER} \
        tiflash:${TIDB_VER} \
        prometheus:${TIDB_VER} \
        grafana:${TIDB_VER}

RUN apt-get -y install mysql-client tzdata

# PD
EXPOSE 2379/tcp

# Prometheus
EXPOSE 9090/tcp

# Grafana
EXPOSE 3000/tcp

# TiDB
EXPOSE 4000/tcp

# TiFlash
EXPOSE 3930/tcp

CMD ["tiup", "playground", "--host=0.0.0.0"]
