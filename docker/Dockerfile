FROM scratch

LABEL name="rdserver"
LABEL version=1.0
LABEL description="Basic hanshake server"

# Copy executable to container
#COPY target/x86_64-unknown-linux-musl/release/rdserver /bin/rdserver
COPY rdserver /bin/rdserver

# run on port 8080
EXPOSE 8080/tcp

ENTRYPOINT [ "/bin/rdserver" ]
