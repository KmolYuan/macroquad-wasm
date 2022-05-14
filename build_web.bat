@echo off
setlocal
set REPO=%~dp0

cargo build --target wasm32-unknown-unknown --release
copy "%REPO%\target\wasm32-unknown-unknown\release\mq-example.wasm" "%REPO%\docs"

echo "Finished"
endlocal
