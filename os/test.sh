for i in 0 1 2 
do 
    cd ../user
    make ch3_$i
    cd ../os
    make run
done