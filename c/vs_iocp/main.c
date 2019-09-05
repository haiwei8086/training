#include <winsock2.h>
#include <ws2tcpip.h>
#include <mswsock.h>
#include <stdio.h>


#pragma comment(lib, "ws2_32.lib")


#define   PORT   5000   
#define   DATA_BUFSIZE   4096   
#define   MAX_THREADS 1

typedef   struct
{
	OVERLAPPED   Overlapped;
	SOCKET   Accept;
	WSABUF   DataBuf;
	CHAR   Buffer[DATA_BUFSIZE];
	DWORD   BytesSEND;
	DWORD   BytesRECV;
	int   Action;

}   PER_IO_OPERATION_DATA, * LPPER_IO_OPERATION_DATA;


typedef   struct
{
	SOCKET   Socket;
	LPFN_ACCEPTEX AcceptEx;
	LPFN_GETACCEPTEXSOCKADDRS GetAcceptSockAddrs;

}   PER_HANDLE_DATA, * LPPER_HANDLE_DATA;


DWORD   WINAPI   ServerWorkerThread(LPVOID   CompletionPortID);
DWORD   WINAPI   SampleWorkerThread(LPVOID   CompletionPortID);

int PostAcceptEx(PER_HANDLE_DATA* handle_data, PER_IO_OPERATION_DATA* PerIoData);
int PostRecv(PER_HANDLE_DATA* handle_data, PER_IO_OPERATION_DATA* PerIoData);
int PostSend(PER_HANDLE_DATA* handle_data, PER_IO_OPERATION_DATA* PerIoData);
int DoAccept(HANDLE CompletionPort, PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData);
int DoRecv(PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData);
int DoSend(PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData);


int main(void)
{
	SOCKADDR_IN   InternetAddr;
	SOCKET   Listen;
	SOCKET   Accept;
	HANDLE   CompletionPort;
	SYSTEM_INFO   SystemInfo;
	LPPER_HANDLE_DATA   PerHandleData;
	LPPER_IO_OPERATION_DATA   PerIoData;
	int   i;
	DWORD   RecvBytes;
	DWORD   Flags;
	DWORD   ThreadID;
	WSADATA   wsaData;
	DWORD   Ret;

	HANDLE   ThreadHandle;
	LPFN_ACCEPTEX lpfnAcceptEx = NULL;
	GUID GuidAcceptEx = WSAID_ACCEPTEX;
	GUID GuidGetAcceptSockAddrs = WSAID_GETACCEPTEXSOCKADDRS;
	int iResult = 0;
	DWORD dwBytes;
	
	int iOptVal = 0;
	int iOptLen = sizeof(int);


	if ((Ret = WSAStartup(0x0202, &wsaData)) != 0)
	{
		printf("WSAStartup() failed. error: %d\n", Ret);
		return;
	}


	// 设置一个I/O完成端口.   
	if ((CompletionPort = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 0)) == NULL)
	{
		printf("CreateIoCompletionPort Failed，err: %d\n", GetLastError());
		return;
	}

	// 测试系统中有多少cpu处理器 
	GetSystemInfo(&SystemInfo);


	// 基于系统可用的处理器创建工作线程，为每个处理器创建连个线程   
	for (i = 0; i < MAX_THREADS; i++)
	{
		// 创建一个服务端线程并且传递一个完成端口给这个线程.   
		if ((ThreadHandle = CreateThread(NULL, 0, SampleWorkerThread, CompletionPort, 0, &ThreadID)) == NULL)
		{
			printf("CreateThread()发生了如下错误： %d\n", GetLastError());
			return;
		}
	}

	//   创建一个监听套接字 
	if ((Listen = WSASocketW(AF_INET, SOCK_STREAM, 0, NULL, 0, WSA_FLAG_OVERLAPPED)) == INVALID_SOCKET)
	{
		printf("WSASocket() 发生了如下错误： %d\n", WSAGetLastError());
		return;
	}
	else
	{
		printf("创建监听套接字成功\n");
	}


	// 创建一个套接字信息结构体去联系起来socket   
	if ((PerHandleData = (LPPER_HANDLE_DATA)GlobalAlloc(GPTR, sizeof(PER_HANDLE_DATA))) == NULL)
	{
		printf("GlobalAlloc()   发生了如下错误：   %d\n", GetLastError());
		return;
	}
	PerHandleData->Socket = Listen;
	PerHandleData->AcceptEx = NULL;
	PerHandleData->GetAcceptSockAddrs = NULL;

	CreateIoCompletionPort((HANDLE)PerHandleData->Socket, CompletionPort, (DWORD)PerHandleData, 0);


	InternetAddr.sin_family = AF_INET;
	InternetAddr.sin_addr.s_addr = htonl(INADDR_ANY);
	InternetAddr.sin_port = htons(PORT);
	if (bind(Listen, (PSOCKADDR)& InternetAddr, sizeof(InternetAddr)) == SOCKET_ERROR)
	{
		printf("bind()端口或IP时发生了如下错误： %d\n", WSAGetLastError());
		return;
	}
	else
	{
		printf("绑定端口%d成功\n", PORT);
	}

	setsockopt(Listen, SOL_SOCKET, SO_KEEPALIVE, (char*)& iOptVal, iOptLen);


	// 准备socket 用来监听   
	if (listen(Listen, 5) == SOCKET_ERROR)
	{
		printf("listen() 发生了如下错误   %d\n", WSAGetLastError());
		return;
	}
	else
	{
		printf("预处理成功，开始在端口 %d 处监听...\n", PORT);
	}

	


	iResult = WSAIoctl(Listen, SIO_GET_EXTENSION_FUNCTION_POINTER, &GuidAcceptEx, sizeof(GuidAcceptEx), &(PerHandleData->AcceptEx), sizeof(PerHandleData->AcceptEx),&dwBytes, NULL, NULL);
	if (iResult == SOCKET_ERROR) {
		wprintf(L"WSAIoctl failed with error: %u\n", WSAGetLastError());
		closesocket(Listen);
		WSACleanup();
		return 1;
	}

	iResult = WSAIoctl(Listen, SIO_GET_EXTENSION_FUNCTION_POINTER, &GuidGetAcceptSockAddrs, sizeof(GuidGetAcceptSockAddrs), &(PerHandleData->GetAcceptSockAddrs), sizeof(PerHandleData->GetAcceptSockAddrs), &dwBytes, NULL, NULL);
	if (iResult == SOCKET_ERROR) {
		wprintf(L"WSAIoctl failed with error: %u\n", WSAGetLastError());
		closesocket(Listen);
		WSACleanup();
		return 1;
	}
	

	for (i = 0; i < MAX_THREADS; i++)
	{
		if ((PerIoData = (LPPER_IO_OPERATION_DATA)GlobalAlloc(GPTR, sizeof(PER_IO_OPERATION_DATA))) == NULL)
		{
			printf("GlobalAlloc() 发生了如下错误： %d\n", GetLastError());
			return;
		}
		ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
		PerIoData->Accept = INVALID_SOCKET;
		PerIoData->BytesSEND = 0;
		PerIoData->BytesRECV = 0;
		PerIoData->DataBuf.len = DATA_BUFSIZE;
		PerIoData->DataBuf.buf = PerIoData->Buffer;
		PerIoData->Action = 0;

		PostAcceptEx(PerHandleData, PerIoData);
	}

	WaitForSingleObject(ThreadHandle, INFINITE);
}


int PostAcceptEx(PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData)
{
	printf("PostAcceptEx \n");

	PerIoData->Action = 0;
	PerIoData->Accept = WSASocketW(AF_INET, SOCK_STREAM, IPPROTO_IP, NULL, 0, WSA_FLAG_OVERLAPPED);

	PerHandleData->AcceptEx(
		PerHandleData->Socket,

		PerIoData->Accept,
		PerIoData->DataBuf.buf,
		0,

		sizeof(SOCKADDR_IN) + 16,
		sizeof(SOCKADDR_IN) + 16,

		&(PerIoData->BytesRECV),
		&(PerIoData->Overlapped)
	);
	
	return 0;
}


int DoAccept(HANDLE CompletionPort, PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData)
{
	printf("DoAccept \n");

	LPPER_HANDLE_DATA   newPerHandleData;
	LPPER_IO_OPERATION_DATA newPerIoData;

	SOCKADDR_IN* RemoteSockAddr = NULL;
	SOCKADDR_IN* LocalSockAddr = NULL;
	int AddrLen = sizeof(SOCKADDR_IN);
	

	setsockopt(PerIoData->Accept, SOL_SOCKET, SO_UPDATE_ACCEPT_CONTEXT, (char*)& PerHandleData->Socket, sizeof(SOCKET));


	PerHandleData->GetAcceptSockAddrs(
		PerIoData->DataBuf.buf,
		PerIoData->DataBuf.len - ((AddrLen + 16) * 2),
		AddrLen + 16, 
		AddrLen + 16,
		(SOCKADDR * *)& LocalSockAddr, &AddrLen,
		(SOCKADDR * *)& RemoteSockAddr, &AddrLen
	);


	if ((newPerHandleData = (LPPER_HANDLE_DATA)GlobalAlloc(GPTR, sizeof(PER_HANDLE_DATA))) == NULL)
	{
		printf("GlobalAlloc()   发生了如下错误：   %d\n", GetLastError());
		return -1;
	}
	newPerHandleData->Socket = PerIoData->Accept;
	newPerHandleData->AcceptEx = PerHandleData->AcceptEx;
	newPerHandleData->GetAcceptSockAddrs = PerHandleData->GetAcceptSockAddrs;


	// Reset PerIoData
	ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
	// PerIoData->Accept = INVALID_SOCKET;
	PerIoData->BytesSEND = 0;
	PerIoData->BytesRECV = 0;
	PerIoData->DataBuf.len = DATA_BUFSIZE;
	PerIoData->DataBuf.buf = PerIoData->Buffer;
	PerIoData->Action = 0;


	PostAcceptEx(PerHandleData, PerIoData);


	if (NULL == CreateIoCompletionPort((HANDLE)newPerHandleData->Socket, CompletionPort, (DWORD)newPerHandleData, 0))
	{
		printf("CreateIoCompletionPort()   发生了如下错误：   %d\n", GetLastError());
		return -1;
	}


	if ((newPerIoData = (LPPER_IO_OPERATION_DATA)GlobalAlloc(GPTR, sizeof(PER_IO_OPERATION_DATA))) == NULL)
	{
		printf("GlobalAlloc() 发生了如下错误： %d\n", GetLastError());
		return;
	}
	ZeroMemory(&(newPerIoData->Overlapped), sizeof(OVERLAPPED));
	newPerIoData->Accept = newPerHandleData->Socket;
	newPerIoData->BytesSEND = 0;
	newPerIoData->BytesRECV = 0;
	newPerIoData->DataBuf.len = DATA_BUFSIZE;
	newPerIoData->DataBuf.buf = newPerIoData->Buffer;
	newPerIoData->Action = 0;


	PostRecv(newPerHandleData, newPerIoData);


	return 0;
}


int PostRecv(PER_HANDLE_DATA* handle_data, PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("PostRecv \n");

	DWORD dwFlags = 0;
	DWORD dwBytes = 0;

	PerIoData->Action = 1;

	WSARecv(PerIoData->Accept, &(PerIoData->DataBuf), 1, &dwBytes, &dwFlags, &(PerIoData->Overlapped), NULL);

	return 0;
}


int DoRecv(PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("Recv data： %s \n", PerIoData->DataBuf.buf);

	ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
	PerIoData->BytesSEND = 0;
	PerIoData->BytesRECV = 0;
	PerIoData->DataBuf.len = DATA_BUFSIZE;
	PerIoData->DataBuf.buf = PerIoData->Buffer;
	PerIoData->Action = 10;


	PostSend(PerHandleData, PerIoData);


	return 0;
}


int PostSend(PER_HANDLE_DATA* handle_data, PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("PostSend \n");

	DWORD dwFlags = 0;
	DWORD dwBytes = 0;
	DWORD SendBytes = 0;
	DWORD Flags;

	char* buf = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n Welcome to Server";
	
	PerIoData->DataBuf.buf = buf;
	PerIoData->DataBuf.len = DATA_BUFSIZE;
	
	//PerIoData->Overlapped.hEvent = WSACreateEvent();
	PerIoData->Action = 2;

	WSASend(PerIoData->Accept, &(PerIoData->DataBuf), 1, &dwBytes, 0, &(PerIoData->Overlapped), NULL);

	printf("PostSend send bytest: %d err: %d\n", dwBytes, GetLastError());

	/*
	WSAWaitForMultipleEvents(1, &PerIoData->Overlapped.hEvent, TRUE, INFINITE, TRUE);
	printf("WSAWaitForMultipleEvents err: %d\n", GetLastError());

	WSAGetOverlappedResult(PerIoData->Accept, &PerIoData->Overlapped, &SendBytes, FALSE, &dwFlags);
	printf("WSAGetOverlappedResult send bytes: %d,  err: %d\n", SendBytes, GetLastError());

	WSAResetEvent(PerIoData->Overlapped.hEvent);
	printf("WSAResetEvent err: %d\n", GetLastError());
	*/

	return 0;
}


int DoSend(PER_HANDLE_DATA* PerHandleData, PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("DoSend \n");

	//closesocket(PerIoData->Accept);
}


DWORD   WINAPI   ServerWorkerThread(LPVOID   CompletionPortID)
{
	HANDLE   CompletionPort = (HANDLE)CompletionPortID;
	DWORD   BytesTransferred;
	LPOVERLAPPED   Overlapped;
	LPPER_HANDLE_DATA   PerHandleData;
	LPPER_IO_OPERATION_DATA   PerIoData;
	DWORD   SendBytes, RecvBytes;
	DWORD   Flags;

	while (TRUE)
	{

		if (GetQueuedCompletionStatus(CompletionPort, &BytesTransferred, (LPDWORD)& PerHandleData, (LPOVERLAPPED*)& PerIoData, INFINITE) == 0)
		{
			printf("GetQueuedCompletionStatus   发生了如下错误： %d\n", GetLastError());
			return   0;
		}

		//首先检查一下去套接字看是否在上发生了错误并且如果发生了错误就关闭套接
		//字并且清除与套接字连接的 SOCKET_INFORMATION结构信息体 
		if (BytesTransferred == 0)
		{
			printf("正在关闭socket   %d\n", PerHandleData->Socket);

			if (closesocket(PerHandleData->Socket) == SOCKET_ERROR)
			{
				printf("closesocket()   发生了如下错误： %d\n", WSAGetLastError());
				return   0;
			}

			GlobalFree(PerHandleData);
			GlobalFree(PerIoData);
			continue;
		}
		//检查如果 BytesRECV字段等于0，这就意味着一个 WSARecv调用刚刚完成了所以从完成的WSARecv()调用中
		//用BytesTransferred值更新 BytesRECV字段 
		if (PerIoData->BytesRECV == 0)
		{
			PerIoData->BytesRECV = BytesTransferred;
			PerIoData->BytesSEND = 0;
		}
		else
		{
			PerIoData->BytesSEND += BytesTransferred;
		}

		if (PerIoData->BytesRECV > PerIoData->BytesSEND)
		{
			//发布另外一个 WSASend()请求
			//既然WSASend()不是 gauranteed去发送所有字节的请求
			//继续调用 WSASend()发送直到所有收到的字节被发送 

			ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));

			PerIoData->DataBuf.buf = PerIoData->Buffer + PerIoData->BytesSEND;
			PerIoData->DataBuf.len = PerIoData->BytesRECV - PerIoData->BytesSEND;

			if (WSASend(PerHandleData->Socket, &(PerIoData->DataBuf), 1, &SendBytes, 0,
				&(PerIoData->Overlapped), NULL) == SOCKET_ERROR)
			{
				if (WSAGetLastError() != ERROR_IO_PENDING)
				{
					printf("WSASend() 发生了如下错误：   %d\n", WSAGetLastError());
					return   0;
				}
			}
		}
		else
		{
			PerIoData->BytesRECV = 0;
			//现在没有更多的字节发送过去用来post另外一个WSARecv()请求 

			Flags = 0;
			ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));

			PerIoData->DataBuf.len = DATA_BUFSIZE;
			PerIoData->DataBuf.buf = PerIoData->Buffer;

			if (WSARecv(PerHandleData->Socket, &(PerIoData->DataBuf), 1, &RecvBytes, &Flags,
				&(PerIoData->Overlapped), NULL) == SOCKET_ERROR)
			{
				if (WSAGetLastError() != ERROR_IO_PENDING)
				{
					printf("WSARecv() 发生了如下错误：   %d\n", WSAGetLastError());
					return   0;
				}
			}
		}
	}
}


DWORD   WINAPI   SampleWorkerThread(LPVOID   CompletionPortID)
{
	HANDLE   CompletionPort = (HANDLE)CompletionPortID;
	DWORD   BytesTransferred;
	LPOVERLAPPED   Overlapped;
	LPPER_HANDLE_DATA   PerHandleData;
	LPPER_IO_OPERATION_DATA   PerIoData;
	DWORD   SendBytes, RecvBytes;
	DWORD   Flags;

	while (TRUE)
	{

		if (GetQueuedCompletionStatus(CompletionPort, &BytesTransferred, (LPDWORD)& PerHandleData, (LPOVERLAPPED*)& PerIoData, INFINITE) == 0)
		{
			printf("GetQueuedCompletionStatus   发生了如下错误： %d\n", GetLastError());
			return   0;
		}

		
		printf("PerIoData->Action： %d\n", PerIoData->Action);

		switch (PerIoData->Action)
		{
		case 0:
			DoAccept(CompletionPort, PerHandleData, PerIoData);
			break;
		case 1:
			DoRecv(PerHandleData, PerIoData);
			break;
		case 2:
			DoSend(PerHandleData, PerIoData);
			break;
		default:

			break;
		}
	}
}