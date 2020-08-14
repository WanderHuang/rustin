import rxjs, { of, Subject, fromEvent, zip } from 'rxjs';
import { takeWhile, mergeMap, map, filter } from 'rxjs/operators';
import test from './test';
import './index.css';

import bubble from './bubble';
import standard from './standard';
import quickSort from './quickSort';

let render$ = new Subject();

let nextVal = (data) => render$.next(data);

import('wasm').then((Algo) => {
  init();
  console.log('测试开始', Algo);

  let val$ = fromEvent(document.querySelector('#count'), 'change').pipe(
    map((e) => Number(e.target.value)),
    filter((val) => !isNaN(val))
  );
  let action$ = fromEvent(document.querySelector('#action'), 'click');

  let test$ = zip(val$, action$);

  test$.subscribe(([val]) => {
    clear();
    console.log('测试长度', val);
    test('冒泡排序', bubble, Algo.bubble)(val).then(nextVal);
    test('快速排序', quickSort, Algo.quickSort)(val).then(nextVal);
    test('语言标准库排序', standard, Algo.standard)(val).then(nextVal);
  });
});

render$.subscribe((next) => {
  console.log('next', next);
  appendRow(next);
  // appendRow(next);
});

function appendRow(res) {
  console.log('appendRow', performance.now());
  let body = document.querySelector('tbody');

  const { name, len, result, timing } = res;

  let row = `
    <tr>
      <td>${name}</td>
      <td>${len}</td>
      <td>${timing[0]}</td>
      <td>${timing[1]}</td>
    </tr>
  `;

  body.innerHTML = body.innerHTML + row;
}

function clear() {
  document.querySelector('tbody').innerText = '';
}

function init() {
  const { appCodeName, appName, appVersion, userAgent } = navigator;

  let obj = { appCodeName, appName, appVersion, userAgent };

  let res = Object.entries(obj).reduce((str, [key, value]) => {
    str += `<div>
          <p>
            <span>${key}</span> : <span>${value}</span>
          </p>
        </div>`;
    return str;
  });

  document.querySelector('#desc').innerHTML = res;
}
