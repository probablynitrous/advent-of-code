# Done as sense check with same logic as powershell 
# solution to see if is just PS thats slow. Shock... it is
def getNodes(content):
  nodes = {}
  for line in content[1:]:
    nodes[line[:3]] = [line[7:10],line[12:15]]
  
  return nodes

def getLCM(a,b):
  if a > b:
    low, high = b,a
  else:
    low, high = a,b

  while (1):
    remainder = high % low
    if (remainder == 0):
      return int((a*b) / low)
    
    high = low
    low = remainder
  
def part1(content):
  directions = content[0].replace("R","1").replace("L","0")
  nodes = getNodes(content)
  currNode = 'AAA'
  steps = 0

  while currNode != 'ZZZ':
    dir = directions[steps%len(directions)]
    currNode = nodes[currNode][int(dir)]
    steps += 1

  print(steps)

def part2(content):
  directions = content[0].replace("R","1").replace("L","0")
  nodes = getNodes(content)

  lcm = 1
  for key in nodes.keys():
    if key[2:] != 'A':
      continue
    currNode = key
    steps = 0
    while currNode[2] != 'Z':   
      dir = directions[steps%len(directions)]
      currNode = nodes[currNode][int(dir)]
      steps += 1
    lcm = getLCM(steps,lcm)
  print(lcm)

content = open("input.txt").read().strip().split('\n')
import time
start_time = time.time()
part1(content)
part2(content)
end_time = time.time()
elapsed_time = end_time - start_time

print(elapsed_time)
