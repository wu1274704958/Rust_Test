#include <iostream>

extern "C" {

	void simple_encode(const char*,char *);
	void simple_decode(const char*,char *);
}


int main()
{
	char out [100] = {0};
	simple_encode("我是谁？",out);
	char out2 [100] = {0};
	simple_decode(out,out2);	
	printf("helloworld %s => %s\n",out,out2);
	return 0;
}
