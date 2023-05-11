import math

def committee():
    read_count = 4 # number of nodes


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

    
    print("Creating Committees: Done")


committee()