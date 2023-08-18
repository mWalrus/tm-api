echo "Compiling API"
cargo build --release

echo "-> Copying service file"
cp "./tm-api.service" /etc/systemd/system/tm-api.service

echo "-> Reloading systemd"
systemctl daemon-reload

echo "-> Starting api service"
systemctl start tm-api && systemctl enable tm-api