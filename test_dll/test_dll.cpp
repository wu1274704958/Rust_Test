#include "test_dll.h"
#include <stdio.h>
#include <iostream>

int say_hello(int a){
    return a << 1;
}

void call_printf(int a)
{
	std::cout << "hello " << a << std::endl;
}
