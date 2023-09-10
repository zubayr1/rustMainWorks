IP=$(curl checkip.amazonaws.com)

ID="null"

# Collect IPs
IPS=()
FILE="${1:-nodes_information.txt}"

# Read ip file
while IFS= read -r line; do
    IPS+=($line)
done < $FILE


# Connect to every IP and run setup.sh on instance
for _ip in "${IPS[@]}"
do
    # Format of ip file NEEDS to be id-ip
    tmp=(${_ip//-/ })
    ip=${tmp[1]}

    if [ "$ip" = "$IP" ]; then
        ID=${tmp[0]}
        fi
done


folder="rustMainWorks"

if [ -d "${folder}" ]; then
  cd "${folder}"
  git stash --include-untracked  # Stash both tracked and untracked changes
  git pull https://github.com/zubayr1/rustMainWorks.git
  git stash pop  # Apply the stashed changes back after the pull
else
  git clone https://github.com/zubayr1/rustMainWorks.git
  cd "${folder}"
fi


# run script
cd "rusting"
python3 Structuring.py


/home/ubuntu/.cargo/bin/cargo run -- keys "$ID" 16 03282129 prod "$IP" 10 1
exit 0