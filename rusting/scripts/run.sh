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

adversary = 0

# check if adversary
FILENEW="${1:-adversaries.txt}"

# Read ip file
while IFS= read -r line; do
     if [[ "$line" == "$IP" ]]; then
        adversary=1
        break  
    fi
done < $FILENEW



cd "rustMainWorks/rusting"

/home/ubuntu/.cargo/bin/cargo run -- nok "$ID" 4 05050021  prod "$IP" 1 "$adversary"
