import { fromEvent, zip } from 'rxjs';
import { map, filter } from 'rxjs/operators';
import './index.css';
import fromWorker from './rx/fromWorker';

function appendRow(res) {
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

function updateDesc(text) {
  document.querySelector('#message').innerText = text;
}

function init() {
  return new Promise((resolve) => {
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
    setTimeout(() => {
      resolve()
    }, 1000)
  })
}

window.onload = function () {
  init().then(() => {
    updateDesc('输入不同的数组长度后点击计算');
    // 事件流
    let test$ = zip(
      fromEvent(document.querySelector('#count'), 'change').pipe(
        map((e) => Number(e.target.value)),
        filter((val) => !isNaN(val))
      ),
      fromEvent(document.querySelector('#action'), 'click')
    );
  
    let worker = new Worker('./worker.js');
  
    let message$ = fromWorker(worker).pipe(
      filter(e => e.data),
      map(e => e.data),
      filter((data) => data.messageType !== 'ready'),
      map(({ messageData }) => messageData)
    );
    message$.subscribe((data) => {
      updateDesc('计算结束');
      appendRow(data);
    });
  
    // 计算
    test$.subscribe(([len]) => {
      updateDesc(`进行计算，数组长度为, ${len}`);
      clear();
      worker.postMessage({ len });
    });
  });
  
};
