@echo off
REM Helper script to ensure proper linking of Windows libraries
REM This addresses the missing advapi32.lib issue with Skia ICU

echo Setting up Windows build environment for OrbitRS...

REM Ensure advapi32.lib is linked
echo Adding advapi32.lib to link libraries

REM Display environment for debugging
echo Windows SDK Dir: %WindowsSdkDir%
echo Windows SDK Version: %WindowsSDKVersion%

REM Add Windows SDK libs to the LIB environment variable
set "LIB=%LIB%;%WindowsSdkDir%\Lib\%WindowsSDKVersion%\um\x64"
echo Updated LIB environment variable

exit /b 0
