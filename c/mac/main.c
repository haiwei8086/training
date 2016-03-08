#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <netdb.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/event.h>
#include <sys/time.h>
#include <err.h>
#include <errno.h>

void die(const char*);
int make_nonblocking(int);

int main(int argc, char *argv[])
{
	int sockfd, nev, kq;

	struct addrinfo hints, *res, *res0;

	struct kevent change[2], event[2];

	ssize_t nbytes;

	int error;

	char buf[BUFSIZ];

	const char *cause = NULL;

	if (3 != argc)	
	{
		fprintf(stderr, "Usage: %s address port\n", argv[0]);
		exit(EXIT_FAILURE);
	}

	(void)memset(&hints, '\0', sizeof(struct addrinfo));

	hints.ai_family = AF_INET;
	hints.ai_socktype = SOCK_STREAM;

	if (0 != (error = getaddrinfo(argv[1], argv[2], &hints, &res0)))
		errx(EXIT_FAILURE, "%s", gai_strerror(error));

	sockfd = -1;

	for (res = res0; res; res = res->ai_next)
	{
		if (-1 == (sockfd = socket(res->ai_family, res->ai_socktype,
		 res->ai_protocol)))
		{
			cause = "socket";
			continue;
		}

		if (-1 == connect(sockfd, res->ai_addr, res->ai_addrlen))
		{
			cause = "connect()";
			close(sockfd);
			sockfd = -1;
			continue;
		}

		break;
		/* NOTREACHED */

	}

	if (-1 == sockfd)
		die(cause);

	freeaddrinfo(res0);

	if (-1 == make_nonblocking(sockfd))
		die("make_nonblock()");

	if (-1 == make_nonblocking(STDIN_FILENO))
		die("make_nonblock()");

	if (-1 == (kq = kqueue()))
		die("kqueue()");

	EV_SET(&change[0], STDIN_FILENO, EVFILT_READ, EV_ADD | EV_ENABLE,
	 0, 0, 0);
	EV_SET(&change[1], sockfd, EVFILT_READ, EV_ADD | EV_ENABLE,
	 0, 0, 0);

	 if (-1 == kevent(kq, change, 2, NULL, 0, NULL))
	 	die("kevent()");

	for (;;)
	{
		printf("blocking in kevent()\n");
		if (-1 == (nev = kevent(kq, NULL, 0, event, 2, NULL)))
			die("kevent()");

		printf("kevent() returned %d\n", nev);

		for (int i = 0; i < nev; i++)
		{
			if (event[i].ident == STDIN_FILENO)
			{
				fgets(buf, sizeof(buf), stdin);

				nbytes = send(sockfd, buf, strlen(buf), 0);

				if (-1 == nbytes)
					if (EWOULDBLOCK != errno)
						die("send()");

			}
			else
			{
				while ((nbytes = recv(sockfd, buf, sizeof(buf), 0)) > 0)
				{

					if (nbytes != write(STDOUT_FILENO, buf, nbytes))
					{
						die("write()");
					}
				}

				if (-1 == nbytes)
				{
					if (EWOULDBLOCK != errno)
						die("recv()");
				}
				else if (EV_EOF & event[i].flags)
				{
					printf("the peer has closed the connection, exiting...\n");
					exit(0);
				}


			}

		}

	}

	return 0;
}

void die(const char *str)
{
	perror(str);
	exit(EXIT_FAILURE);
}

int make_nonblocking(int fd)
{
	int flags;
	if (-1 == (flags = fcntl(fd, F_GETFL)))
		return -1;

	flags |= O_NONBLOCK;

	if (-1 == fcntl(fd, F_SETFL, flags))
		return -1;

	return 0;

}
