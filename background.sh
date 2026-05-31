cargo build --release --bin background
nohup target/release/background > background.log 2>&1 &