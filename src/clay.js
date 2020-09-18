import { Subject, Observable, fromEvent } from 'rxjs';
import { pluck, filter, takeUntil, timeout } from 'rxjs/operators';
function Clay(opts = {}, middlewares = []) {
  this.channel = new Subject();

  let message$ = fromEvent(window || self, 'message');

  message$.pipe(
    pluck('data'),
    filter(predict => predict.uid),
  ).subscribe(next => {
    this.channel.next(next);
  })
}

let ActionResultType =  Object.freeze({
  Success: 1,
  Fail: 2,
  Timeout: 3
})

Clay.prototype.post = (message, origin, { uid, timer = 3000 } = {}) => {
  let next = middlewares.filter(fn => typeof fn === 'function').reduce((prev, curr) => curr(prev), message);
  uid = uid ? uid :uuid();
  this.sent.add(uid);
  postTo({ message: next, uid }, origin);

  return new Promise((resolve, reject) => {
    let listen$ = this.channel.pipe(
      filter(predict => predict.uid === uid),
      takeUntil(timeout(timer)),
    );
    let subscription = listen$.subscribe(
      next => {
        resolve(wrapResult(next.message, ActionResultType.Success));
        subscription.unsubscribe();
      },
      err => reject(wrapResult(err.message, ActionResultType.Fail)),
      completation => resolve(wrapResult(null, ActionResultType.Timeout))
    )
  });
};

// let cancel = clay.listen(fn)
// cancel();
Clay.prototype.listen = (fn) => {
  return this.channel.subscribe(next => {
    // each listen will watch every message `Clay` received
    fn(next.message);
  })
}


function postTo(data, origin = '*') {
  if (window?.postMessage) {
    window.postMessage(data, origin)
  } else if (self?.postMessage) {
    self.postMessage(data, origin)
  }
}

function uuid () {
  return Array.from({ length: 36 }).fill(0).reduce(
    (uid, _) => 
      uid + 
      (uid.length > 0 && uid.length % 8 === 0 ?
      '-' :
      '0123456789abcdef'[((Math.random() * 0x10) | 0)])
    , '');
}

function wrapResult(message, status) {
  return {
    message,
    status
  }
}

export default Clay;