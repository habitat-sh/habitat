import { Pipe, PipeTransform } from '@angular/core';

@Pipe({ name: 'habKeysPipe', pure: false })
export class KeysPipe implements PipeTransform {
  transform(value: any, args: any[] = null): any {
    return Object.keys(value);
  }
}
