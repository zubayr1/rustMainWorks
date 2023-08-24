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
    echo $ip
    scp -i "randpiper.pem" /home/zake/newrand/nodes_information.txt /home/zake/newrand/adversaries.txt /home/zake/newrand/setup.sh /home/zake/newrand/run.sh ubuntu@$ip:/home/ubuntu
    ssh -i "randpiper.pem" ubuntu@$ip "bash setup.sh" &
done