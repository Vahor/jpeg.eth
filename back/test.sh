docker build -f server.Dockerfile -t legendora-server . --platform=linux/arm64

docker save -o back.tar legendora-server

scp back.tar root@172.104.149.133:/root/back.tar