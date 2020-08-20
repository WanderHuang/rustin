import { Subject, Observable } from 'rxjs';
import { map, filter } from 'rxjs/operators';

export default function fromWorker(worker) {
  let message$ = new Subject();
  worker.onmessage = (e) => message$.next(e);

  return new Observable(observer => {
      message$.subscribe(val => observer.next(val));
  })
}

export function createWorkerMessage(src = './worker.js') {
  let worker = new Worker(src);
  return [
    worker,
    fromWorker(worker).pipe(
      filter(e => e.data),
      map(e => e.data),
      filter((data) => data.messageType !== 'ready'),
      map(({ messageData }) => messageData)
    )
  ];
}