#ifndef __TEST_DLL_H__
#define __TEST_DLL_H__

#define TEST_DLL_EXPORT __attribute__((visibility("default")))

extern "C" {

    TEST_DLL_EXPORT int say_hello(int );
    TEST_DLL_EXPORT void call_printf( );
}

#endif //__TEST_DLL_H__