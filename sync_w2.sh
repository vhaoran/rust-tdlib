

ssh whr@w2 "cd /D d:\rust\; rmdir rust-tdlib /D /Q"
rm -rf ./target

scp -r ../rust-tdlib  whr@w2:/d:/rust/


