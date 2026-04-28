


rm -rf ./target

ssh whr@w3 "cd /D d:\rust\; rmdir rust-tdlib /D /Q"

scp -r ../rust-tdlib  whr@w3:/d:/rust/


