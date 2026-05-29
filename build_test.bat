@echo off
call "D:\vis_installer\VC\Auxiliary\Build\vcvars64.bat"
cd /d e:\.list
cl.exe /EHsc /utf-8 /std:c++17 tests\test_ffi.cpp /I tests /link /LIBPATH:target\release dotlist.lib ws2_32.lib user32.lib kernel32.lib advapi32.lib ntdll.lib userenv.lib secur32.lib /OUT:output\tests\test_ffi.exe
pause
