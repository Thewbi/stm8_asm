cls
@ECHO OFF

set input_file=res/C/samples/c_samples/array_0.c
CALL "target/debug/rust_pest_asm.exe" -i "%input_file%"

REM echo test 1
REM call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
REM echo test 2
ml64.exe /c main.asm >NUL 2>&1
REM echo test 3
link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO /ENTRY:main main.obj >NUL 2>&1
REM echo test 4
REM echo calling main.exe
REM for /f %%a in ('main.exe') do set "dow=%%a"
REM %dow%
REM echo blibb: %dow%
CALL main.exe
SET bbb=%ERRORLEVEL%
REM bbb = %ERRORLEVEL%
REM echo %bbb%
REM echo Return Value: %bbb%
REM echo test 5
IF "%bbb%" == "0" ( echo [SUCCESS] %input_file% ) ELSE ( echo [FAILURE] %input_file% )

set input_file=res/C/samples/c_samples/array_1.c
CALL "target/debug/rust_pest_asm.exe" -i "%input_file%"
ml64.exe /c main.asm >NUL 2>&1
link.exe /SUBSYSTEM:console /LARGEADDRESSAWARE:NO /ENTRY:main main.obj >NUL 2>&1
CALL main.exe
SET bbb=%ERRORLEVEL%
IF "%bbb%" == "0" ( echo [SUCCESS] %input_file% ) ELSE ( echo [FAILURE] %input_file% )

echo [DONE]
