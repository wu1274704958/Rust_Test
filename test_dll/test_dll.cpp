#include "test_dll.h"
#include <stdio.h>

int say_hello(int a){
    return a << 1;
}

void call_printf()
{
    printf("hello\n");
}
