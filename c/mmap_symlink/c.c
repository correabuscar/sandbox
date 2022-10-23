//taken from: https://bugzilla.kernel.org/show_bug.cgi?id=2743

/*this test case will test the various calls related with shared
memort segments */

#include <stdio.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <errno.h>

#ifndef PAGESIZE
#define PAGESIZE 4096
#endif

int main(int argc, char *argv[]){
	key_t k;
	int project_id=9;
	int segment;
	int pagesize;
	int fd;
	struct stat st;
	int status;
	char * file, * end_of_file;
	char * shared_mm;

//create a key for shared memory segment

	if((k = ftok(argv[1],project_id))==-1)
	{
		perror("error generating key\n\n");
	}//end of ftok
	

	
	//allocate a shared memory segment.... get a identifier for it.
	pagesize=getpagesize();
	
	if((segment = shmget(k,2*pagesize,IPC_CREAT)) <0)
	{
		printf("Error no :: %d\n", errno);
		perror("Error in shmget ");
	}//end of shmget
	
	printf("Segment id :: %d\n", segment);
	
	//we need to attach the shared memory segment in process address space.

	//we will do this after a mapping of a file.... 
	//it should pass.
	
	//open a file first
	if((fd = open(argv[1],O_RDWR))==-1)
	{
		perror("opening of file failed");
	}//end of open
	
	//get the size of file
	
	if((status = fstat(fd,&st))==-1)
	{
		perror("error getting the file stats");
	}//end of fstat
	
	//mmap the file
	
	if((file = (char *)mmap(NULL,st.st_size,PROT_READ|PROT_WRITE,MAP_SHARED,fd,0))==-1)
	{
		perror("error mapping the file");
	}//end of mmap call
	
	//file is pointer to the mapped memory .... shared segment to be attached at 
	//the end of file mapping
	
	//locate the end of file and make it page aligned
	
	end_of_file = file + st.st_size;
	
	printf("end of file is located at %x\n\n",end_of_file);
	
	end_of_file = (char *)(((int)end_of_file + PAGESIZE-1)&~(PAGESIZE-1));
	
	printf("end of file after alligning is located at %x\n\n",end_of_file);
	
	
	//execute the shmat
	if((shared_mm = shmat(segment,(void *)end_of_file,1))==-1)
	{
		perror("The shmat call failed");
	}//end of shmat call
	
	printf("The address returned by shmat is %x\n\n",shared_mm);
	
	
	}//end of main
