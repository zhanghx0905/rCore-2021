for i in 0 1 2 
do 
    cd ../user
    make all CHAPTER=3_$i
    cd ../os
    make run
done