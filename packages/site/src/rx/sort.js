import { fromEvent, zip, merge } from 'rxjs';
import { map, filter } from 'rxjs/operators';
import fromDomEvent from './fromDomEvent';

export default () => zip(
  fromDomEvent('#sortLength', 'change'),
  merge(
    fromDomEvent('#sortAction', 'click'),
    fromEvent(window, 'keydown').pipe(
      map(e => e.keyCode),
      filter(val => val === 13)
    )
  )
)