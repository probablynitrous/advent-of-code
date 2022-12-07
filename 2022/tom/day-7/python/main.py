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
  data = open(file_name, "r").readlines()
  return data

def step1(data):
  dirs, _ = build_dirs(data)
          
  total = 0
  for dir in dirs:
    if dirs[dir] <= 100000:
      total += dirs[dir]
  
  return total

def step2(data):
  dirs, total_size = build_dirs(data)
          
  space_required = 30000000 - (70000000 - total_size)
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
    match line.split():
      case '$', 'cd', '..':
        curr_path.pop()
      case '$', 'cd', drive:
        curr_path.append(drive)
      case '$',_:
        pass
      case 'dir',_:
        pass
      case size, _:
        dirs, total_size = update_path(dirs,curr_path,size,total_size)
      
  
  return dirs,total_size

def update_path(dirs, curr_path, size, total_size):
  value = int(size)
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