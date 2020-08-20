import './index.css';
import { createWorkerMessage } from './rx/fromWorker';
import createSort from './rx/sort';
import createMd5 from './rx/md5';
import createClear from './rx/clear';
import md5 from './md5/index';
import { init, clear, updateDesc, appendRow } from './fns/dom';

window.onload = function () {
  
  init().then(() => {
    let sortTest$ = createSort();
    let md5$ = createMd5();
    let clear$ = createClear();
    updateDesc('输入不同的长度后点击计算');

    // 消息处理
    let [worker, message$] = createWorkerMessage();
    message$.subscribe((data) => {
      updateDesc('计算结束');
      appendRow(data);
    });

    // 发起测试
    sortTest$.subscribe(([len]) => {
      updateDesc(`进行计算，数组长度为, ${len}`);
      worker.postMessage({ len });
    });

    md5$.subscribe(([len]) => {
      updateDesc(`md5计算中，字符串长度为，${len}`);
      md5(len).then(res => {
        let row = {
          name: 'MD5计算',
          len,
          timing: [res.jsTime, res.wasmTime],
          result: [res.jsRes, res.wasmRes],
          count: res.count
        }
        console.log('res', row, 'input', res.strs);
        appendRow(row);
      });
    });

    // 按钮功能
    clear$.subscribe(() => {
      clear();
    });
  });
};
