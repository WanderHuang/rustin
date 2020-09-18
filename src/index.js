import Clay from './clay';

let clay = new Clay();

clay.listen((message) => {
  console.log('[ clay ] > main', message);
});