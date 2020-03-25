Command to run go-agent agent:
(replace the IP address with the right go-server address. Check using `docker inspect` on the server)
```bash
sudo docker run -d -e GO_SERVER_URL=https://172.17.0.3:8154/go akshayde/gocd-agent-rust:latest
```

Command to run server:
```bash
sudo docker run -d -p8153:8153 -p8154:8154 -v /storage/data/godata:/godata -v /home/go:/home/go --user root:root gocd/gocd-server:v19.12.0
```
