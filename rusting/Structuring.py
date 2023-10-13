import math


def committee():
    read_count = 64 # number of nodes

    file = open("nodes_information.txt", "r")

    filew = open("updatednodeinfo.txt", "w")

    print("Creating Committees: Start")

    height = math.log(read_count,2)

    level = 1

    tcount = 1

    lis = []

    # Open the file and read it in each iteration
    with open("nodes_information.txt", "r") as file:
        
        while height >= 0:
            file.seek(0)
            count = 0
            for f in file:
                id = f.rstrip().split("-")[0]
                                               
                lis.append(tcount)
                if int(id) % level == 0:                    
                    tcount+=1
                
                count+=1
            
            height -= 1
            level = level*2
            tcount+=1
                       

    
    with open("nodes_information.txt", "r") as file:
        with open("updatednodeinfo.txt", "w") as filew:
            file.seek(0)
            start=0
            for f in file:
                val = f.rstrip()
                for i in range(start, len(lis), read_count): 
                    val = val+ " "+ str(lis[i])
                filew.write(val)
                filew.write("\n")
                start+=1

    

    with open("updatednodeinfo.txt", 'r') as file:
        input_string = file.read()

    # Process the input string
    lines = input_string.split('\n')

    j = 0
    output_lines = []
    for line in lines:
        parts = line.split(' ')
        new_parts = [parts[0]]
        j+=1
        for i in range(0, len(parts)-1):
            
            
            if 2**i==1:
                new_parts.append(parts[i+1] + 'l')
            else:
                
                committee_num = 0
                if (2**i)>=j:
                    committee_num = 1
                else:
                    committee_num = (math.ceil(j/(2**i)))

                temp = j
                temp = j - ((2**i)*(committee_num-1))
                               

                if temp<=(2**i)/2:
                    new_parts.append(parts[i+1] + 'l')
                else:
                    new_parts.append(parts[i+1] + 'r')
        output_lines.append(' '.join(new_parts))

    output_string = '\n'.join(output_lines)

    # Write the updated content back to the file
    with open("updatednodeinfo.txt", 'w') as file:
        file.write(output_string)
    
    print("Creating Committees: Done")


committee()