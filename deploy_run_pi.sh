echo "Copying files"
echo "1/4"
scp -r ./target/arm-unknown-linux-gnueabihf/release/* pi@ssh.jornvanwier.com:~/Rust/static_fileserver
echo "2/4"
scp -r ./templates pi@ssh.jornvanwier.com:~/Rust/static_fileserver
echo "3/4"
scp -r ./static pi@ssh.jornvanwier.com:~/Rust/static_fileserver
echo "4/4"
scp -r ./Rocket.toml pi@ssh.jornvanwier.com:~/Rust/static_fileserver

echo "Executing"
ssh -t pi@ssh.jornvanwier.com "cd ~/Rust/static_fileserver && sudo ROCKET_ENV=production ~/Rust/static_fileserver/static_fileserver"