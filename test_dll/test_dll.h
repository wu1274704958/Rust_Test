#ifndef __TEST_DLL_H__
#define __TEST_DLL_H__

extern "C" {

__attribute__((visibility("default"))) int say_hello(int );

}

#endif //__TEST_DLL_H__