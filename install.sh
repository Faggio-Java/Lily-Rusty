sudo mv lily /usr/
sudo chown $(whoami) /usr/lily/
cargo install --path .
echo "Lily installed, p.s: make sure .cargo is in your PATH"
