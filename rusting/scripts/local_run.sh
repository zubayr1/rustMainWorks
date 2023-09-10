# Collect IPs
IPS=()
FILE="${1:-nodes_information.txt}"

# Read ip file
while IFS= read -r line; do
    IPS+=($line)
done < "$FILE"

# Function to introduce a 10ms delay
delay() {
    local n=$1
    while [ "$n" -gt 0 ]; do
        sleep 0.01  # Sleep for 10 milliseconds
        n=$((n-1))
    done
}

# Connect to every IP and run run.sh on instance
for _ip in "${IPS[@]}"; do
    # Format of ip file NEEDS to be id-ip
    tmp=(${_ip//-/ })
    ip=${tmp[1]}
    echo "$ip"
    ssh -i "randpiper.pem" ubuntu@"$ip" "bash run.sh" &
    delay 1  # Introduce a 10ms delay before the next SSH connection
done
