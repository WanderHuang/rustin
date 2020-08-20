import { fromEvent, zip, merge } from 'rxjs';
import { map, filter } from 'rxjs/operators';
import fromDomEvent from './fromDomEvent';

export default () => zip(
  fromDomEvent('#md5Length', 'change'),
  fromDomEvent('#md5Count', 'change'),
  merge(
    fromDomEvent('#md5Action', 'click'),
    fromEvent(window, 'keydown').pipe(
      map(e => e.keyCode),
      filter(val => val === 13)
    )
  )
);
