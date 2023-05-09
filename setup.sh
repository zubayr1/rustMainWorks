folder="rustMainWorks"

if [ -d "${folder}" ]; then
  cd "${folder}"
  git stash
  git pull https://github.com/zubayr1/rustMainWorks.git
else
  git clone https://github.com/zubayr1/rustMainWorks.git
  cd "${folder}"
fi


# run script
cd "rusting"
/home/ubuntu/.cargo/bin/cargo run -- nok {1} 4 03282129 prod {18.117.92.19} 4 0