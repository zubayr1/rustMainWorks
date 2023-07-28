import math

def committee():
    read_count = 16 # number of nodes


    file = open("nodes_information.txt", "r")

    filew = open("updatednodeinfo.txt", "w")

    filew = open("tempnodeinfo.txt", "w")

    print("Creating Committees: Start")

    MINGROUPCOUNT = 1

    count_groupid = 1
    count_entries = MINGROUPCOUNT
    
    height = math.log(read_count,2) + 1

    
    while(height >0):
        for f in file:
            s = f.rstrip() + " " + str(count_groupid) + "\n"
            #print(s)
            filew.write(s)

            count_entries-=1

            if count_entries ==0:
                count_entries = MINGROUPCOUNT
                count_groupid +=1
            
        height -=1
        
        MINGROUPCOUNT*=2
        count_entries = MINGROUPCOUNT
        
        file.close()
        filew.close()
        
        file = open("tempnodeinfo.txt", "r")
        filew = open("tempnodeinfo.txt", "a")

        #print("------------------------")
              

    filew = open("updatednodeinfo.txt", "a")


    total_len=0
    with open("tempnodeinfo.txt", "r") as fp:
        total_len = len(fp.readlines())

    total_len = total_len - read_count

    for f in open("tempnodeinfo.txt", "r").readlines():
        
        if total_len<=0:
            filew.write(f)
            read_count-=1
            
            if read_count==0:
                break
        total_len-=1

    filew.close()
    
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