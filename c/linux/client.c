#include <netinet/in.h>
#include <sys/socket.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <sys/types.h>
#include <arpa/inet.h>

#define MAXSIZE     1024 * 2
#define IPADDRESS   "192.168.0.40"
#define SERV_PORT   9000

int main(int argc,char *argv[])
{
    int                 sockfd;
    struct sockaddr_in  servaddr;
    char* buf = "GET / HTTP/1.0\r\nHost: 127.0.0.1:9000\r\nUser-Agent: ApacheBench/2.3\r\nAccept: */*\r\n\r\n";
    char recv_buf[MAXSIZE];
    memset(recv_buf,0,MAXSIZE);

    sockfd = socket(AF_INET,SOCK_STREAM,0);
    bzero(&servaddr,sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_port = htons(SERV_PORT);
    inet_pton(AF_INET,IPADDRESS,&servaddr.sin_addr);
    connect(sockfd,(struct sockaddr*)&servaddr,sizeof(servaddr));
    //处理连接
    send(sockfd, buf, strlen(buf), 0);
    recv(sockfd, recv_buf, sizeof(recv_buf), 0);

    printf("%s\n", recv_buf);

    shutdown(sockfd, SHUT_RD);
    return 0;
}
