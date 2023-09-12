# Collect IPs
IPS=()
FILE="${1:-nodes_information.txt}"

# Read ip file
while IFS= read -r line; do
    IPS+=($line)
done < "$FILE"



# Connect to every IP and run run.sh on instance
for _ip in "${IPS[@]}"; do
    # Format of ip file NEEDS to be id-ip
    tmp=(${_ip//-/ })
    ip=${tmp[1]}
    echo "$ip"
    ssh -i "randpiper.pem" ubuntu@"$ip" "bash run.sh" &
done
