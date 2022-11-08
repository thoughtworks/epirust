#! /bin/bash
logsFolder=$1
totalEngines=$(ls "$logsFolder" | wc -l)
#echo "$totalEngines"

echo "EngineId,""Iter/sec,""totalTime,""receiveTickWaitTime,""receiveCommutersWaitTime,""receiveMigratorsWaitTime,""sendCommutersWaitTime,""sendMigratorsWaitTime"
for ((i = 0; i < totalEngines; i++)); do
  currentFile="$logsFolder/${i}_logs.txt"
  currentEngineId=$(($i+1))
  totalTime=$(tail "$currentFile" | head -n 1 | awk -F 'Total Time taken' '{print $2 }' | awk -F ' ' '{print $1}')
  iterationPerSecond=$(tail -n 9 "$currentFile" | head -n 1 | awk -F 'Iterations/sec: ' '{print $2 }' | awk -F ' ' '{print $1}')
  receiveTickWaitTime=$(tail -n 8 "$currentFile" | head -n 1 | awk -F 'total tick sync time: ' '{print $2 }' | awk -F ' ' '{print $1}')
  receiveCommutersWaitTime=$(tail -n 7 "$currentFile" | head -n 1 | awk -F 'total receive commute sync time:' '{print $2 }' | awk -F ' ' '{print $1}')
  receiveMigratorsWaitTime=$(tail -n 6 "$currentFile" | head -n 1 | awk -F 'total receive migration sync time: ' '{print $2 }' | awk -F ' ' '{print $1}')
  sendCommutersWaitTime=$(tail -n 5 "$currentFile" | head -n 1 | awk -F 'total send commuters sync time: ' '{print $2 }' | awk -F ' ' '{print $1}')
  sendMigratorsWaitTime=$(tail -n 4 "$currentFile" | head -n 1 | awk -F 'total send migrators sync time: ' '{print $2 }' | awk -F ' ' '{print $1}')
  echo "Engine$currentEngineId",$iterationPerSecond,$totalTime,$(($receiveTickWaitTime / 1000)),$(($receiveCommutersWaitTime / 1000)),$(($receiveMigratorsWaitTime / 1000)),$(($sendCommutersWaitTime / 1000)),$(($sendMigratorsWaitTime / 1000))
done
