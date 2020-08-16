import { Subject, Observable } from 'rxjs';

export default function fromWorker(worker) {
  let message$ = new Subject();
  worker.onmessage = (e) => message$.next(e);

  return new Observable(observer => {
      message$.subscribe(val => observer.next(val));
  })
}
