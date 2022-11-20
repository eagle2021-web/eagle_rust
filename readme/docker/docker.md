##### 创建容器执行一个简单的命令
```bash
docker run ubuntu:15.10 /bin/echo "Hello world"
```

##### 退出容器，但是不让容器停止
ctrl + p and ctrl + q

#### mysql镜像
```shell
docker pull mysql:latest

docker run -itd --name mysql-test -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 mysql
```