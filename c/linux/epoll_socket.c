
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

#include <netinet/in.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <sys/epoll.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>

#define IP                      "0.0.0.0"
#define PORT                    9000
#define MAX_BUFFER_SIZE         2 * 1024
#define MAX_RECV_BUF_SIZE       2 * 1024
#define MAX_SEND_BUF_SIZE       2 * 1024
#define MAX_LISTEN_QUEUE        5
#define MAX_EVENTS              1024


//函数声明
//创建套接字并进行绑定
static int socket_listen(const char* ip,int port);

// set socket opts
static void set_socket_opts(int sockfd);

// set non blocking
static void make_non_blocking(int sockfd);

//事件处理函数
static void eventloop(int listenfd);
//处理接收到的连接
static void handle_accept(int epollfd, int sockfd);
//读处理
static void handle_read(int epollfd,int fd,char *buf);
//写处理
static void handle_write(int epollfd,int fd,char *buf);
//添加事件
static void add_event(int epollfd,int fd,int state);
//修改事件
static void modify_event(int epollfd,int fd,int state);
//删除事件
static void delete_event(int epollfd,int fd,int state);

int main() {
    int listenfd;

    listenfd = socket_listen(IP, PORT);

    printf("Server running on %d\n", PORT);

    eventloop(listenfd);

    return 0;
}

static int socket_listen(const char* ip, int port) {
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
    // addr.sin_addr.s_addr = htonl(INADDR_ANY);
    inet_pton(AF_INET, ip, &addr.sin_addr);
    addr.sin_port = htons(port);

    if(bind(listenfd, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        perror("Bind socket faield!");
        exit(1);
    }

    if(listen(listenfd, MAX_LISTEN_QUEUE) < 0) {
        perror("Socket listen failed!");
        exit(1);
    }

    make_non_blocking(listenfd);

    return listenfd;
}

static void set_socket_opts(int sockfd) {
    struct linger so_linger;
    int keepalive, reuse;
    socklen_t keepalive_len, reuse_len, so_linger_len;

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
    // Revice buffer size
    int n_recv_buf = MAX_RECV_BUF_SIZE;
    if(setsockopt(sockfd, SOL_SOCKET, SO_RCVBUF, (const char*)&n_recv_buf, sizeof(int)) < 0) {
        perror("Set socket recv buffer size failed!");
    }else{
        printf("Set socket recv buffer size succeed\n");
    }
    // send buffer size
    int n_send_buf = MAX_SEND_BUF_SIZE;
    if(setsockopt(sockfd, SOL_SOCKET, SO_SNDBUF, (const char*)&n_send_buf, sizeof(int)) < 0) {
        perror("Set socket send buffer size failed!");
    }else{
        printf("Set socket send buffer size succeed\n");
    }
    // keep alive
    keepalive = 1;
    if(setsockopt(sockfd, SOL_SOCKET, SO_KEEPALIVE, (const char*)&keepalive, sizeof(int)) < 0) {
        perror("Set socket keepalive failed!");
    }else{
        printf("Set socket keepalive succeed\n");
    }
    // re-use addr
    reuse = 1;
    if(setsockopt(sockfd, SOL_SOCKET, SO_REUSEADDR, (const char*)&reuse, sizeof(int)) < 0) {
        perror("Set socket reuse addr failed!");
    }else{
        printf("Set socket reuse addr succeed\n");
    }
    // linger
    so_linger.l_onoff = 1;
    so_linger.l_linger = 5;
    if(setsockopt(sockfd, SOL_SOCKET, SO_LINGER, (const char*)&so_linger, sizeof(&so_linger)) < 0) {
        perror("Set socket linger failed!");
    }else{
        printf("Set socket linger succeed\n");
    }
}

static void make_non_blocking(int sockfd) {
    int flags = fcntl(sockfd, F_GETFL, 0);
    if(flags < 0) {
        perror("Get socket non-blocking failed!");
        flags = 0;
    }
    if(fcntl(sockfd, F_SETFL, flags | O_NONBLOCK) < 0){
        perror("Get socket non-blocking failed!");
    }else{
        printf("Set socket non-blocking succeed\n");
    }
}

static void eventloop(int listenfd) {
    int epfd, ret;
    struct epoll_event ev, events[MAX_EVENTS];
    char buf[MAX_BUFFER_SIZE];
    char* response = "Welcome to epoll server.";

    memset(&buf, 0, MAX_BUFFER_SIZE);

    epfd = epoll_create1(0);
    add_event(epfd, listenfd, EPOLLIN);

    while(1) {
        ret = epoll_wait(epfd, events, MAX_EVENTS, -1);
        if(ret == -1) {
            perror("Epoll wait failed");
            exit(EXIT_FAILURE);
        }

        for(int i = 0; i < ret; i++) {

            if((events[i].events == EPOLLERR || events[i].events == EPOLLHUP) &&
                !(events[i].events == EPOLLIN))
            {
                printf("Epoll error: %d\n", events[i].events);
                close(events[i].data.fd);
                continue;
            } else if ((events[i].data.fd == listenfd) && (events[i].events & EPOLLIN)) {
                handle_accept(epfd, listenfd);
            } else if (events[i].events == EPOLLIN) {
                handle_read(epfd, events[i].data.fd, buf);
            } else if (events[i].events == EPOLLOUT) {
                handle_write(epfd, events[i].data.fd, response);
            }
        }
    }
}

static void handle_accept(int epfd, int sockfd) {
    int clientfd;
    struct sockaddr_in clientaddr;
    socklen_t clientaddr_len;

    clientfd = accept(sockfd, (struct sockaddr*)&clientaddr, &clientaddr_len);
    if(clientfd == -1) {
        if((errno == EAGAIN) ||
            (errno == EWOULDBLOCK) ||
            (errno == ECONNABORTED) ||
            (errno == EPROTO) ||
            (errno == EINTR)) {
            /* We has processed incoming connections. */
            perror("Accpet: allow error.");
            return;
        } else {
            perror("Socket accpet failed");
            return;
        }
    }

    printf("Accpet a connection: IP: %s, Port: %d\n", inet_ntoa(clientaddr.sin_addr), clientaddr.sin_port);

    make_non_blocking(clientfd);
    add_event(epfd, clientfd, EPOLLIN | EPOLLET);
}

static void handle_read(int epfd, int fd, char* buf) {
    int n_read;
    n_read = read(fd, buf, MAX_BUFFER_SIZE);

    if(n_read == -1 && n_read != EAGAIN) {
        perror("Read failed!");
        close(fd);
        return;
    } else if (n_read == 0) {
        perror("End of fd, the remote has closed the connection");
        close(fd);
        return;
    }

    printf("Read count: %d\n", n_read);
    printf("Read content: %s\n", buf);

    modify_event(epfd, fd, EPOLLOUT | EPOLLET);
}

static void handle_write(int epfd, int fd, char* buf) {
    int n_write, buf_len = strlen(buf), need_len;
    need_len = buf_len;

    n_write = write(fd, buf, buf_len);
    if(n_write == -1) {
        if((n_write == EINTR) || (n_write == EAGAIN) || (n_write == EWOULDBLOCK)) {
            perror("Write blocked or eintr, need re-write.");
        }else{
            perror("Write failed!");
            close(fd);
            return;
        }
    }

    printf("Write count: %d\n", buf_len);
    printf("Write end --------------------------------\n");
    close(fd);
}


static void add_event(int epollfd,int fd,int state)
{
    struct epoll_event ev;
    ev.events = state;
    ev.data.fd = fd;
    epoll_ctl(epollfd,EPOLL_CTL_ADD,fd,&ev);
}

static void delete_event(int epollfd,int fd,int state)
{
    struct epoll_event ev;
    ev.events = state;
    ev.data.fd = fd;
    epoll_ctl(epollfd,EPOLL_CTL_DEL,fd,&ev);
}

static void modify_event(int epollfd,int fd,int state)
{
    struct epoll_event ev;
    ev.events = state;
    ev.data.fd = fd;
    epoll_ctl(epollfd,EPOLL_CTL_MOD,fd,&ev);
}
