//src: https://wiki.strongswan.org/issues/990

#include <pthread.h>

#include <stdio.h>

#include <stdlib.h>

#include <string.h>

#include <sys/types.h>

#define THREADS        128

#define MALLOC_SIZE    10240

void *fork_test(void *none)

{

        char *a;

    int status;

    pid_t pid;

    while (1) {

        if (!(pid = fork())) {

            a = malloc(MALLOC_SIZE);

            memset(a, 1, MALLOC_SIZE);

            free(a);

                    exit(0);

            }

        if (pid < 0) {

            printf("Failed to fork\n");

            exit(1);

        }

        wait();

        a = malloc(MALLOC_SIZE);

        memset(a, 1, MALLOC_SIZE);

        free(a);

    }

    return NULL;

}

int main()

{

    pthread_t threads[THREADS];

    int i;

    for (i = 0; i < THREADS; i++) {

        if (pthread_create(&threads[i], NULL, fork_test, NULL))

            printf("Failed to create thread %u\n", i);

    }

    pause();

    return 0;

}

