#ifndef __TEST_DLL_H__
#define __TEST_DLL_H__

#ifdef _MSC_VER 
	#define TEST_DLL_EXPORT _declspec(dllexport)
#else 
	#define TEST_DLL_EXPORT __attribute__((visibility("default")))
#endif

extern "C" {

    TEST_DLL_EXPORT int say_hello(int );
    TEST_DLL_EXPORT void call_printf( int );
}

#endif //__TEST_DLL_H__