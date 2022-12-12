import * as fs from 'fs';
import test from 'node:test';
import { inspect } from 'util';

type Monkey = {
  items: number[],
  operation: Operation, 
  test: {
    value: number,
    true: number,
    false: number
  }
  inspected: number
}

type Operation = {
  value:string,
  operator:string
}

async function main() {
	var data:String[];
  data = readFile();
  const step1Result = await step1(data);
  console.log("Step 1: ", step1Result);
  const step2Result = await step2(data);
  console.log("Step 2: ", step2Result);
}

function readFile():String[] {
  var contents
  if (process.argv.slice(2).includes("test")) {
		contents = fs.readFileSync('../test.txt', 'utf-8');
	} else {
		contents = fs.readFileSync('../input.txt', 'utf-8');
	}
  const data:String[] = contents.split('\r\n\r\n');
  return data;
}

async function step1(data:String[]):Promise<number>
{
  let monkeys: Monkey[] = await parseMonkey(data);
  for (let i = 0; i < 20; i++)  {
    for (let i = 0; i < monkeys.length; i++) {
      const monkey = monkeys[i];
      for (let k = 0; k < monkey.items.length; k++) {
        const item = monkey.items[k];
        let newValue = await getNewValue(item,monkey.operation);
          
        newValue = await Math.floor(newValue / 3);
        if ((newValue % monkey.test.value) == 0) {
          monkeys[monkey.test.true].items.push(newValue);
        } else {
          monkeys[monkey.test.false].items.push(newValue);
        }
        monkeys[i].inspected += 1;
      }
      monkey.items = []
    }
  }

  monkeys.sort((x,y) => y.inspected-x.inspected);

  monkeys.slice(0,2)

  
  return monkeys[0].inspected * monkeys[1].inspected;
}

async function step2(data:String[]):Promise<number>
{
  let monkeys: Monkey[] = await parseMonkey(data);
  const denominator = monkeys.map(m => m.test.value).reduce((a, b) => a * b)
  for (let i = 0; i < 10000; i++)  {
    for (let i = 0; i < monkeys.length; i++) {
      const monkey = monkeys[i];
      for (let k = 0; k < monkey.items.length; k++) {
        const item = monkey.items[k];
        let newValue = await getNewValue(item,monkey.operation);

        newValue = newValue % denominator
          
        newValue = await Math.floor(newValue);
        if ((newValue % monkey.test.value) == 0) {
          monkeys[monkey.test.true].items.push(newValue);
        } else {
          monkeys[monkey.test.false].items.push(newValue);
        }
        monkeys[i].inspected += 1;
      }
      monkey.items = []
    }
  }


  monkeys.sort((x,y) => y.inspected-x.inspected);

  monkeys.slice(0,2)

  
  return monkeys[0].inspected * monkeys[1].inspected;
}

async function getNewValue(value: number, operation: Operation): Promise<number> {
  let newValue = 0;
  switch (operation.operator) {
    case '*':
      if (+operation.value) {
        newValue = value * +operation.value;
      } else {
        newValue = value * value;
      }
      break;
    case '+':
      if (+operation.value) {
        newValue = value + +operation.value;
      } else {
        newValue = value + value;
      }
      break;
  }   
  return newValue
}

async function parseMonkey(data: String[]): Promise<Monkey[]> {
  let monkeys: Monkey[] = [];
  for await (const monkeyBlock of data) {
    let monkeyArray = monkeyBlock.split('\r\n');
    let startingItems:number[] = await parseItems(monkeyArray[1].split(':')[1].split(','));
    let operation:Operation = await pasreOperator(monkeyArray[2].split(':')[1]);
    let testValue = +monkeyArray[3].match(/[0-9]+$/)![0]
    let testTrue = +monkeyArray[4].match(/[0-9]+$/)![0]
    let testFalse = +monkeyArray[5].match(/[0-9]+$/)![0]

    let monkey:Monkey = {
      items: startingItems,
      operation: operation,
      test: {
        value: testValue,
        true: testTrue,
        false: testFalse
      },
      inspected: 0
    }
    monkeys.push(monkey)
  }
  return monkeys
}

async function parseItems(items:String[]):Promise<number[]> 
{
  let itemList:number[] = [];
  for await (const item of items) {
    itemList.push(+item)
  }
  return itemList
}

async function pasreOperator(operationString: String):Promise<Operation> {
  let operation = {} as Operation; 
  let m = operationString.match(/= old (.) (.+)/)
  operation.operator = m![1]
  operation.value = m![2];

  return operation
}
main()