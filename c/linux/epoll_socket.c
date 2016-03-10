
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#include <netinet/in.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <unistd.h>
#include <sys/types.h>

#define IP                  "0.0.0.0"
#define PORT                9000
#define MAX_BUFFER_SIZE     2 * 1024
#define MAX_RECV_BUF_SIZE   2 * 1024
#define MAX_SEND_BUF_SIZE   2 * 1024
#define LISTEN_QUEUE        5
#define MAX_EVENTS          1024


//函数声明
//创建套接字并进行绑定
static int socket_bind(const char* ip,int port);

// set socket opts
static void set_socket_opts(int listenfd);

// set non blocking
static int make_non_blocking(int listenfd);

//IO多路复用epoll
static void do_epoll(int listenfd);
//事件处理函数
static void
handle_events(int epollfd,struct epoll_event *events,int num,int listenfd,char *buf);
//处理接收到的连接
static void handle_accpet(int epollfd,int listenfd);
//读处理
static void do_read(int epollfd,int fd,char *buf);
//写处理
static void do_write(int epollfd,int fd,char *buf);
//添加事件
static void add_event(int epollfd,int fd,int state);
//修改事件
static void modify_event(int epollfd,int fd,int state);
//删除事件
static void delete_event(int epollfd,int fd,int state);

int main() {
    int listenfd;
    listenfd = socket_bind(IP, PORT);

    printf("Server running on %d", PORT);
}

static int socket_bind(const char* ip, int port) {
    int listenfd;
    struct sockaddr_in addr;

    listenfd = socket(AF_INET, SOCK_STREAM, 0);
    if(listenfd < 0) {
        perror("Create socket failed!");
        exit(1);
    }

    set_socket_opts(listenfd);

    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    inet_pton(AF_INET, ip, &addr.sin_addr);
    addr.sin_port = htons(port);

    if(bind(listenfd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("Bind socket faield!");
        exit(1);
    }

    return listenfd;
}

static void set_socket_opts(int sockfd) {
    struct linger so_linger;
    int keepalive, reuse, keepalive_len;
    socklen_t reuse_len, l_len, so_linger_len;

    if(getsockopt(sockfd, SOL_SOCKET, SO_KEEPALIVE , &keepalive, &keepalive_len) < 0){
        perror("Get socket option faield: SO_KEEPALIVE");
        return;
    }
    if(getsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR , &reuse, &reuse_len) < 0){
        perror("Get socket option faield: SO_REUSEADDR");
        return;
    }
    if(getsockopt(sockfd, SOL_SOCKET, SO_LINGER , &so_linger, &so_linger_len) < 0){
        perror("Get socket option faield: SO_LINGER");
        return;
    }

    // set options
    //接收缓冲区
    int n_recv_buf= MAX_RECV_BUF_SIZE; //设置为32K
    setsockopt(sockfd, SOL_SOCKET, SO_RCVBUF, (const char*)&n_recv_buf, sizeof(int));
    //发送缓冲区
    int n_send_buf = MAX_SEND_BUF_SIZE; //设置为32K
    setsockopt(sockfd, SOL_SOCKET, SO_SNDBUF, (const char*)&n_send_buf, sizeof(int));

    printf("SO_KEEPALIVE: %d\n", keepalive);
    printf("SO_REUSEADDR: %d\n", reuse);
    printf("SO_LINGER.l_onoff : %d\n", so_linger.l_onoff);
    printf("SO_LINGER.l_linger  : %d\n", so_linger.l_linger);

}
