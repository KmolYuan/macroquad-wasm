@echo off
setlocal
set REPO=%~dp0

cargo install basic-http-server
basic-http-server --addr 127.0.0.1:8080 "%REPO%\docs"
endlocal
