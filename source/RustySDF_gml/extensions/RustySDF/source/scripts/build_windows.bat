@echo off
REM ##### extgen :: user entrypoint (IfMissing — customize freely) #####
REM Regenerated core lives in scripts\extgen\ — this wrapper is yours.
REM Core deploys RustySDF.dll to targets.windows.outputFolder ("..").

REM If CARGO_TARGET_DIR is set externally to another folder, deploy would copy
REM the wrong DLL. Always build into this crate's local target.
set "CARGO_TARGET_DIR=%~dp0..\rust\target"

call "%~dp0extgen\build_windows.bat" %*
if errorlevel 1 exit /b 1
