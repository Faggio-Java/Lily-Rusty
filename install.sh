sudo mv -r lily /usr/
sudo chown $(whoami) /usr/lily/
sudo echo "alias lily=/usr/lily/package.sh" >> ~/.bashrc && . ~/.bashrc
cargo install --path .
echo "Lily installed, p.s: make sure .cargo is in your PATH"