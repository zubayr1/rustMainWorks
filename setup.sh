folder="rustBeacon"
if [ -d "${folder}" ]; then
  cd "${folder}"
  git pull https://github.com/zubayr1/rustBeacon.git
else
  git clone https://github.com/zubayr1/rustBeacon.git
  cd "${folder}"
fi


# run script
cd "rusting"
cargo run -- nok {1} 4 03282129 prod {18.117.92.19} 4 0