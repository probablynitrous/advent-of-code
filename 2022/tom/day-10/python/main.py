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

signal = 0

def read_file(file_name):
  data = open(file_name, "r").readlines()
  return data

def step1(data):
  i = 1
  total = 1
  for line in data:
    i += 1
    update_cycle(i, total)
    match line.split():
      case 'addx',value:
        i += 1
        total += int(value)
        update_cycle(i, total)
  global signal
  return signal
  

def step2(data):
  i = 1
  total = 1
  crt_line = ''
  for line in data:
    if pixel_exists(i,total):
      crt_line += '#'
    else:
      crt_line += '.'
    crt_line = check_if_print(i,crt_line)
    if crt_line == '':
      i = 0
    i += 1
    match line.split():
      case 'addx',value:
        if pixel_exists(i,total):
          crt_line += '#'
        else:
          crt_line += '.'
        crt_line = check_if_print(i,crt_line)
        if crt_line == '':
          i = 0
        total += int(value)
        i += 1
        
  
  print(crt_line)

def update_cycle(i, total):
  if i in [20,60,100,140,180,220]:
    global signal
    signal += i * total

def pixel_exists(i, total):
  if i in range(total,total+3):
    return True
  return False

def check_if_print(i, crt_line):
  if i in [40,80,120,160,200,240]:
    print(crt_line)
    return ''
  return crt_line

if __name__ == '__main__':
    sys.exit(main())