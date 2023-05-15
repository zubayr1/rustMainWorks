# Collect IPs
IPS=()
FILE="${1:-nodes_information.txt}"

# NOTE: format inside file NEEDS to be id-ip
while IFS= read -r line; do
    IPS+="$(echo "${line}" | cut -d "-" -f 2) "
done < $FILE

# Connect to every IP and run local run.sh
for ip in "${IPS[@]}"
do
    echo $ip
done