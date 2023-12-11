def part1(content):
  result = 0
  for line in content:
    result += next(line)
  print(result)
def part2(content):
  result = 0
  for line in content:
    result += prev(line)
  print(result)

def prev(line):
  history = line.split(' ')
  totaldiff = loop(history, 0, False)
  return totaldiff + int(history[0])

def next(line):
  history = line.split(' ')
  totaldiff = loop(history, 0, True)
  return totaldiff + int(history[-1])
 
def loop(line, totaldiff,forward):
  i = 1
  diff = []
  while i < len(line):
    diff.append(int(line[i]) - int(line[i - 1]))
    i += 1
  if True in (value != 0 for value in diff):
    totaldiff += loop(diff, totaldiff, forward) * (1 if forward else -1)
  if forward:
    totaldiff += diff[-1]
  else:
    totaldiff -= diff[0]
  return totaldiff


content = open("input.txt").read().strip().split('\n')
import time
start_time = time.time()
part1(content)
part2(content)
end_time = time.time()
elapsed_time = end_time - start_time

print(elapsed_time)