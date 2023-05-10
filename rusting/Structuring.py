def committee():
    file = open("nodes_information.txt", "r")

    filew = open("updatednodeinfo.txt", "w")

    filew = open("tempnodeinfo.txt", "w")

    print("Creating Committees: Start")

    MINGROUPCOUNT = 1

    count_groupid = 1
    count_entries = MINGROUPCOUNT

    total_count = 3
    read_count = 4 # number of nodes

    while(total_count >0):
        for f in file:
            s = f.rstrip() + " " + str(count_groupid) + "\n"
            #print(s)
            filew.write(s)

            count_entries-=1

            if count_entries ==0:
                count_entries = MINGROUPCOUNT
                count_groupid +=1
            
        total_count -=1
        
        MINGROUPCOUNT*=2
        count_entries = MINGROUPCOUNT
        
        file.close()
        filew.close()
        
        file = open("tempnodeinfo.txt", "r")
        filew = open("tempnodeinfo.txt", "a")

        #print("------------------------")
        

        

    filew = open("updatednodeinfo.txt", "a")



    for f in reversed(open("tempnodeinfo.txt", "r").readlines()):
        filew.write(f)
        read_count-=1
        
        if read_count==0:
            break

    print("Creating Committees: Done")


committee()