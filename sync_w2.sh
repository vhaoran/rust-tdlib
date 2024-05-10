


rm -rf ./target

ssh whr@w2 "cd /D d:\rust\; rmdir rust-tdlib /D /Q"

scp -r ../rust-tdlib  whr@w2:/d:/rust/


