import rxjs, { of, Subject } from 'rxjs';
import test from './test';
import './index.css';

import bubble from './bubble';
import standard from './standard';


let render$ = new Subject();

import('wasm').then((Algo) => {
  console.log('测试开始', Algo)
  // 冒泡排序测试
  let bubbleTest = test('bubble', bubble, Algo.bubble);
  render$.next(bubbleTest(10));
  render$.next(bubbleTest(10000));
  render$.next(bubbleTest(10000000));

  console.log('冒泡排序 > 测试完毕')

  let standardTest = test('standard', standard, Algo.standard);
  render$.next(standardTest(10));
  render$.next(standardTest(10000));
  render$.next(standardTest(10000000));

  console.log('标准库排序 > 测试完毕')
});

render$.subscribe((next) => {
  console.log('next', next);
  appendRow(next);
});


function appendRow(res) {
  let body = document.querySelector('tbody');

  const {name, len, result, timing} = res;

  let row = `
    <tr>
      <td>${name}</td>
      <td>${len}</td>
      <td>${timing[0]}</td>
      <td>${timing[1]}</td>
    </tr>
  `;

  body.innerHTML = body.innerHTML + row
}