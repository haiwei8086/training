#include <winsock2.h>
#include <ws2tcpip.h>
#include <mswsock.h>
#include <stdio.h>


#pragma comment(lib, "ws2_32.lib")


#define   PORT   5000   
#define   DATA_BUFSIZE   4096   
#define   MAX_THREADS 1


static GUID GuidAcceptEx			= WSAID_ACCEPTEX;
static GUID GuidGetAcceptSockAddrs	= WSAID_GETACCEPTEXSOCKADDRS;

LPFN_ACCEPTEX				lpfnAcceptEx;
LPFN_GETACCEPTEXSOCKADDRS	lpfnGetAcceptSockAddrs;

HANDLE						CompletionPort;
SOCKET                      ListenSocket;


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


DWORD   WINAPI   SampleWorkerThread(LPVOID   CompletionPortID);


int PostAcceptEx(PER_IO_OPERATION_DATA* PerIoData);
int PostRecv(PER_IO_OPERATION_DATA* PerIoData);
int PostSend(PER_IO_OPERATION_DATA* PerIoData);

int DoAccept(HANDLE CompletionPort, PER_IO_OPERATION_DATA* PerIoData);
int DoRecv(PER_IO_OPERATION_DATA* PerIoData);
int DoSend(PER_IO_OPERATION_DATA* PerIoData);



int main2(void)
{	
	SOCKADDR_IN					InternetAddr;
	SYSTEM_INFO					SystemInfo;
	LPPER_IO_OPERATION_DATA		PerIoData;

	DWORD		Ret;
	int			i;
	DWORD		ThreadID;
	WSADATA		wsaData;
	HANDLE		ThreadHandle;
	
	int iResult = 0;
	DWORD dwBytes;
	
	int iOptVal = 1;
	int iOptLen = sizeof(int);


	if ((Ret = WSAStartup(0x0202, &wsaData)) != 0)
	{
		printf("WSAStartup() failed. error: %d\n", Ret);
		return -1;
	}
	if (LOBYTE(wsaData.wVersion) != 2 || HIBYTE(wsaData.wVersion) != 2) 
	{
		printf("Request Windows Socket Version 2.2 Error!\n");
		WSACleanup();
		return -1;
	}

	if ((CompletionPort = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, 0, 0)) == NULL)
	{
		printf("CreateIoCompletionPort Failed£¬err: %d\n", GetLastError());
		return -1;
	}


	GetSystemInfo(&SystemInfo);


	for (i = 0; i < MAX_THREADS; i++)
	{  
		if ((ThreadHandle = CreateThread(NULL, 0, SampleWorkerThread, CompletionPort, 0, &ThreadID)) == NULL)
		{
			printf("CreateThread() failed. error: %d\n", GetLastError());
			WSACleanup();
			return -1;
		}
	}

	
	if ((ListenSocket = WSASocketW(AF_INET, SOCK_STREAM, 0, NULL, 0, WSA_FLAG_OVERLAPPED)) == INVALID_SOCKET)
	{
		printf("WSASocket() failed. error: %d\n", WSAGetLastError());
		WSACleanup();
		return -1;
	}

	CreateIoCompletionPort((HANDLE)ListenSocket, CompletionPort, 0, 0);


	setsockopt(ListenSocket, SOL_SOCKET, SO_KEEPALIVE, (char*)& iOptVal, iOptLen);

	InternetAddr.sin_family = AF_INET;
	InternetAddr.sin_addr.s_addr = htonl(INADDR_ANY);
	InternetAddr.sin_port = htons(PORT);
	if (bind(ListenSocket, (PSOCKADDR)& InternetAddr, sizeof(InternetAddr)) == SOCKET_ERROR)
	{
		printf("bind() failed. error: %d\n", WSAGetLastError());

		closesocket(ListenSocket);
		WSACleanup();
		CloseHandle(CompletionPort);
		return -1;
	}


	if (listen(ListenSocket, SOMAXCONN) == SOCKET_ERROR)
	{
		printf("listen() failed. error: %d\n", WSAGetLastError());
		
		closesocket(ListenSocket);
		WSACleanup();
		CloseHandle(CompletionPort);
		return -1;
	}


	printf("IOCP: %p, Socket: %I64d \n", CompletionPort, ListenSocket);
	printf("Scoket listen on port: %d \n", PORT);
	

	iResult = WSAIoctl(
		ListenSocket,
		SIO_GET_EXTENSION_FUNCTION_POINTER, 
		&GuidAcceptEx, 
		sizeof(GUID),
		&lpfnAcceptEx,
		sizeof(LPFN_ACCEPTEX),
		&dwBytes, 
		NULL, 
		NULL);

	if (iResult == SOCKET_ERROR) {
		wprintf(L"WSAIoctl failed. error: %u\n", WSAGetLastError());

		closesocket(ListenSocket);
		WSACleanup();
		CloseHandle(CompletionPort);
		return -1;
	}

	iResult = WSAIoctl(
		ListenSocket,
		SIO_GET_EXTENSION_FUNCTION_POINTER, 
		&GuidGetAcceptSockAddrs, 
		sizeof(GUID),
		&lpfnGetAcceptSockAddrs,
		sizeof(LPFN_GETACCEPTEXSOCKADDRS),
		&dwBytes, 
		NULL, 
		NULL);

	if (iResult == SOCKET_ERROR) {
		wprintf(L"WSAIoctl failed. error: %u\n", WSAGetLastError());
		
		closesocket(ListenSocket);
		WSACleanup();
		CloseHandle(CompletionPort);
		return -1;
	}
	

	for (i = 0; i < MAX_THREADS; i++)
	{
		if ((PerIoData = (LPPER_IO_OPERATION_DATA)GlobalAlloc(GPTR, sizeof(PER_IO_OPERATION_DATA))) == NULL)
		{
			printf("GlobalAlloc() failed. error: %d\n", GetLastError());

			closesocket(ListenSocket);
			WSACleanup();
			CloseHandle(CompletionPort);
			return -1;
		}
		ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
		PerIoData->Accept = INVALID_SOCKET;
		PerIoData->BytesSEND = 0;
		PerIoData->BytesRECV = 0;
		PerIoData->DataBuf.len = DATA_BUFSIZE;
		PerIoData->DataBuf.buf = PerIoData->Buffer;
		PerIoData->Action = 0;

		PostAcceptEx(PerIoData);
	}



	WaitForSingleObject(ThreadHandle, INFINITE);

	return 0;
}


int PostAcceptEx(PER_IO_OPERATION_DATA* PerIoData)
{
	printf("PostAcceptEx. PerIoData: %p \n", PerIoData);

	PerIoData->Action = 0;
	PerIoData->Accept = WSASocketW(AF_INET, SOCK_STREAM, 0, NULL, 0, WSA_FLAG_OVERLAPPED);

	lpfnAcceptEx(
		ListenSocket,

		PerIoData->Accept,
		PerIoData->DataBuf.buf,
		0,

		sizeof(SOCKADDR_IN) + 16,
		sizeof(SOCKADDR_IN) + 16,

		&(PerIoData->BytesRECV),
		&(PerIoData->Overlapped)
	);

	printf("PostAcceptEx(). last error: %d\n", GetLastError());
		
	return 0;
}


int DoAccept(HANDLE CompletionPort, PER_IO_OPERATION_DATA* PerIoData)
{
	printf("DoAccept. PerIoData: %p \n", PerIoData);

	SOCKET client_socket;
	LPPER_IO_OPERATION_DATA newPerIoData;

	SOCKADDR_IN* RemoteSockAddr = NULL;
	SOCKADDR_IN* LocalSockAddr = NULL;
	int AddrLen = sizeof(SOCKADDR_IN);


	client_socket = PerIoData->Accept;

	if (setsockopt(client_socket, SOL_SOCKET, SO_UPDATE_ACCEPT_CONTEXT, (char*)& ListenSocket, sizeof(SOCKET)) == -1)
	{
		printf("setsockopt(SO_UPDATE_ACCEPT_CONTEXT) failed. error: %d\n", GetLastError());
	}

	lpfnGetAcceptSockAddrs(
		PerIoData->DataBuf.buf,
		PerIoData->DataBuf.len - ((AddrLen + 16) * 2),
		AddrLen + 16, 
		AddrLen + 16,
		(SOCKADDR * *)& LocalSockAddr, &AddrLen,
		(SOCKADDR * *)& RemoteSockAddr, &AddrLen
	);

	// Reset PerIoData
	ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
	// PerIoData->Accept = INVALID_SOCKET;
	PerIoData->BytesSEND = 0;
	PerIoData->BytesRECV = 0;
	PerIoData->DataBuf.len = DATA_BUFSIZE;
	PerIoData->DataBuf.buf = PerIoData->Buffer;
	PerIoData->Action = 0;


	PostAcceptEx(PerIoData);


	if (NULL == CreateIoCompletionPort((HANDLE)client_socket, CompletionPort, 0, 0))
	{
		printf("CreateIoCompletionPort() failed. error: %d\n", GetLastError());
		return -1;
	}


	if ((newPerIoData = (LPPER_IO_OPERATION_DATA)GlobalAlloc(GPTR, sizeof(PER_IO_OPERATION_DATA))) == NULL)
	{
		printf("GlobalAlloc() faild. error: %d\n", GetLastError());
		return -1;
	}
	ZeroMemory(&(newPerIoData->Overlapped), sizeof(OVERLAPPED));
	newPerIoData->Accept = client_socket;
	newPerIoData->BytesSEND = 0;
	newPerIoData->BytesRECV = 0;
	newPerIoData->DataBuf.len = DATA_BUFSIZE;
	newPerIoData->DataBuf.buf = newPerIoData->Buffer;
	newPerIoData->Action = 0;


	PostRecv(newPerIoData);

	return 0;
}


int PostRecv(PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("PostRecv \n");

	DWORD dwFlags = 0;
	DWORD dwBytes = 0;

	PerIoData->Action = 1;

	WSARecv(PerIoData->Accept, &(PerIoData->DataBuf), 1, &dwBytes, &dwFlags, &(PerIoData->Overlapped), NULL);

	return 0;
}


int DoRecv(PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("Recv data£º %s \n", PerIoData->DataBuf.buf);

	ZeroMemory(&(PerIoData->Overlapped), sizeof(OVERLAPPED));
	PerIoData->BytesSEND = 0;
	PerIoData->BytesRECV = 0;
	PerIoData->DataBuf.len = DATA_BUFSIZE;
	PerIoData->DataBuf.buf = PerIoData->Buffer;
	PerIoData->Action = 10;

	PostSend(PerIoData);

	return 0;
}


int PostSend(PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("PostSend \n");

	DWORD dwFlags = 0;
	DWORD dwBytes = 0;
	DWORD SendBytes = 0;

	char bufs[] = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nWelcome to Server.";
	
	PerIoData->DataBuf.buf = &bufs;
	PerIoData->DataBuf.len = sizeof(bufs) - 1;
	
	PerIoData->Overlapped.hEvent = WSACreateEvent();
	PerIoData->Action = 2;

	WSASend(PerIoData->Accept, &(PerIoData->DataBuf), 1, &dwBytes, 0, &(PerIoData->Overlapped), NULL);

	printf("PostSend send bytest: %d err: %d\n", dwBytes, GetLastError());

	
	WSAWaitForMultipleEvents(1, &PerIoData->Overlapped.hEvent, TRUE, INFINITE, TRUE);
	printf("WSAWaitForMultipleEvents err: %d\n", GetLastError());

	WSAGetOverlappedResult(PerIoData->Accept, &PerIoData->Overlapped, &SendBytes, FALSE, &dwFlags);
	printf("WSAGetOverlappedResult send bytes: %d,  err: %d\n", SendBytes, GetLastError());

	WSAResetEvent(PerIoData->Overlapped.hEvent);
	printf("WSAResetEvent err: %d\n", GetLastError());


	return 0;
}


int DoSend(PER_IO_OPERATION_DATA* PerIoData) 
{
	printf("DoSend \n");

	shutdown(PerIoData->Accept, SD_BOTH);
	GlobalFree(PerIoData);
	return 0;
}


DWORD   WINAPI   SampleWorkerThread(LPVOID   Params)
{
	HANDLE CompletionPort = (HANDLE)Params;
	DWORD  BytesTransferred;
	PULONG_PTR  lpCompletionKey = NULL;

	LPOVERLAPPED lpOverlapped;
	LPPER_IO_OPERATION_DATA  PerIoData;


	while (TRUE)
	{
		if (GetQueuedCompletionStatus(CompletionPort, &BytesTransferred, &lpCompletionKey, (LPOVERLAPPED*)& PerIoData, INFINITE) == 0)
		{
			printf("GetQueuedCompletionStatus() failed. error: %d\n", GetLastError());
			return   0;
		}
		//PerIoData = (LPPER_IO_OPERATION_DATA)CONTAINING_RECORD(lpOverlapped, PER_IO_OPERATION_DATA, Overlapped);


		printf("GetQueuedCompletionStatus. PerIoData: %p \n", PerIoData);
		

		printf("Bytes transferred: %d \n", BytesTransferred);
		printf("PerIoData->Action£º %d\n", PerIoData->Action);

		
		switch (PerIoData->Action)
		{
		case 0:
			DoAccept(CompletionPort, PerIoData);
			break;
		case 1:
			DoRecv(PerIoData);
			break;
		case 2:
			DoSend(PerIoData);
			break;
		default:

			break;
		}
	}
}