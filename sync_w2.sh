


rm -rf ./target

sshpass -p ww ssh whr@w2 "cd /D d:\rust\; rmdir rust-tdlib /D /Q"

sshpass -p ww scp -r ../rust-tdlib  whr@w2:/d:/rust/


