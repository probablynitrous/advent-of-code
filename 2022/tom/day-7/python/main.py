import re
import sys

def main() : 
	data = []
	if 'test' in sys.argv:
		data = read_file('../test.txt')
	else:
		data = read_file('../input.txt')

	step1_result = step1(data)
	print ("Step 1: ", step1_result)
	step2_result = step2(data)
	print ("Step 2: ", step2_result)


def read_file(file_name):
  contents = open(file_name, "r")
  data = contents.read().split('\n')
  return data

def step1(data):
  result = build_dirs(data)
  dirs = result[0]
          
  total = 0
  for dir in dirs:
    if dirs[dir] <= 100000:
      total += dirs[dir]
  
  return total

def step2(data):
  result = build_dirs(data)
  dirs = result[0]
  total_size = result[1]
          
  unsused_space = 70000000 - total_size
  space_required = 30000000 - unsused_space
  curr_delete_size = total_size
  for dir in dirs:
    if dirs[dir] >= space_required:
      if curr_delete_size > space_required:
        curr_delete_size = dirs[dir]
  
  return curr_delete_size

def build_dirs(data):
  dirs = {}
  curr_path = []
  total_size = 0
  for line in data:
    m = re.match('\$ cd (?!\.\.)(.+)', line)
    if m:
      curr_path.append(m.group(1))

    if line == '$ cd ..':
      curr_path.pop()

    m = re.match('([0-9]+) (.+)', line)
    if m:
      value = int(m.group(1))
      total_size += value
      curr_path_string = ''
      for dir in curr_path:
        curr_path_string = curr_path_string + '/' + dir
        if curr_path_string in dirs:
          dirs[curr_path_string] += value
        else:
          dirs[curr_path_string] = value
  
  return dirs,total_size

if __name__ == '__main__':
    sys.exit(main())