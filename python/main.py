#!/usr/bin/python
from fileinput import close
import sys
import time
from datetime import datetime

start = time.time_ns()

query = sys.argv[1]
file = sys.argv[2]

def search(query, file):
    results = []
    with open(file) as FileObj:
        for line in FileObj:
            if query in line:
                results.append(line)
    return results

results = search(query, file)

end = time.time_ns()
print('matches:', len(results))
print('operation complete in', round((end-start)/1000000, 2), 'ms')
