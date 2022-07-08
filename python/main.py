#!/usr/bin/python
from fileinput import close
import sys
import time
from datetime import datetime

start = time.time_ns()

query = sys.argv[1]
file = sys.argv[2]

data = open(file, "r").readlines()

def search(query, data):
    results = []
    for line in data:
        if query in line:
            results.append(line)
    return results

results = search(query, data)

end = time.time_ns()
print('matches:', len(results))
print('operation complete in', round((end-start)/1000000, 2), 'ms')
