```shell
VERSION_STRING=20.10.9
yum install docker-ce-${VERSION_STRING} docker-ce-cli-${VERSION_STRING} containerd.io
```

```shell
docker run --help
  -p, --publish list                   Publish a container's port(s) to the host
  -P, --publish-all                    Publish all exposed ports to random ports
  
docker run -i -t -d --name aaa ubuntu /bin/bash
docker run -d --p 5000:5000 --name a2 training/webapp python app.py
```

配置 DNS
我们可以在宿主机的 /etc/docker/daemon.json 文件中增加以下内容来设置全部容器的 DNS：
```json
{
  "dns" : [
    "114.114.114.114",
    "8.8.8.8"
  ]
}
```

构建镜像
我们使用命令 docker build ， 从零开始来创建一个新的镜像。为此，我们需要创建一个 Dockerfile 文件，其中包含一组指令来告诉 Docker 如何构建我们的镜像。

runoob@runoob:~$ cat Dockerfile
```dockerfile
FROM    centos:7.6
MAINTAINER      Fisher "fisher@sudops.com"

RUN     /bin/echo 'root:123456' | chpasswd
RUN     useradd runoob
RUN     /bin/echo 'runoob:123456' | chpasswd
RUN     /bin/echo -e "LANG=\"en_US.UTF-8\"" >/etc/default/local
EXPOSE  22
EXPOSE  80
CMD     /usr/sbin/sshd -D
```