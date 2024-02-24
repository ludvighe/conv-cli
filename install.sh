target_path="target/release/conv"
install_path="/usr/bin/"
cargo build --release && sudo cp $target_path $install_path && echo -e "\n✓ Conv installed to /usr/bin/"