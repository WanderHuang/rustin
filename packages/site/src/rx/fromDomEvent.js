import { fromEvent, zip, merge } from 'rxjs';
import { map, filter, debounceTime } from 'rxjs/operators';

export default function (tag, event) {
  let dom = document.querySelector(tag);
  let tagName = dom.tagName;

  let ob$ = fromEvent(dom, event);
  if (tagName === 'INPUT') {
    ob$ = ob$.pipe(
      map((e) => Number(e.target.value)),
      filter((val) => !isNaN(val))
    )
  }

  return ob$.pipe(debounceTime(300))
}