import hashlib

def sha256_hash(node_identifier):
    # Calculate the SHA-256 hash of the node identifier
    hash_object = hashlib.sha256(node_identifier.encode('utf-8'))
    hex_digest = hash_object.hexdigest()
    # Convert the hexadecimal hash to an integer and adjust to the desired port range
    return int(hex_digest, 16)

def hash_node_to_port(node_identifier, port_range_start, port_range_end):
    hash_value = sha256_hash(node_identifier)  # Use the SHA-256 hash function
    port_range = port_range_end - port_range_start + 1
    port = port_range_start + (hash_value % port_range)  # Adjust to the desired port range
    return port

def create_ports(port_str, port_range_start, port_range_end):
    return hash_node_to_port(port_str, port_range_start, port_range_end)



def write_port(port_list, file_name):
    with open(file_name, "w") as file:
        # Convert all elements to strings using map() and join them with newline
        lines = map(str, port_list)
        file.writelines("\n".join(lines))


def portify(my_port, n, port_range_start, port_range_end):
    
    server_port_list = []
    client_port_list = []
    
    for i in range(n):
        server_str = str(my_port)+str(i+1)
        server_port_list.append(create_ports(server_str, port_range_start, port_range_end))
        
        client_str = str(i+1)+str(my_port)
        client_port_list.append(create_ports(client_str, port_range_start, port_range_end))
        
    
    write_port(server_port_list, "server_port_list" + my_port+ ".txt")
    write_port(client_port_list, "client_port_list" + my_port+ ".txt")
    print("ports created")



def run():
    file = open("nodes_information.txt", "r")

    lines = file.readlines()

    for line in lines:
        parts = line.strip().split("-")

        portify(parts[0], 4, 5000, 7000)
    
    
run()