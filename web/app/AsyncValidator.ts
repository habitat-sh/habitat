// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Observable} from "rxjs";
import {Observer} from "rxjs/Observer";
import "rxjs/add/operator/debounceTime";
import "rxjs/add/operator/distinctUntilChanged";
import {Control} from "angular2/common";

// Wraps an async validator with a static `debounce` method, so you can debounce
// async validation.
//
// Where you would normally put:
//
//     myAsyncValidator
//
// Use:
//
//     AsyncValidator.debounce(control => myAsyncValidator(control))
//
// Taken from http://stackoverflow.com/a/36076946.
export class AsyncValidator {
    private validate;

    constructor(validator: (control: Control) => any, debounceTime = 300) {
        let source: any = new Observable((observer: Observer<Control>) => {
            this.validate = (control) => observer.next(control);
        });

        source.debounceTime(debounceTime)
            .distinctUntilChanged(null, (x) => x.control.value)
            .map(x => { return { promise: validator(x.control), resolver: x.promiseResolver }; })
            .subscribe(
            (x) => x.promise.then(resultValue => x.resolver(resultValue),
                (e) => { console.log("async validator error: %s", e); }));
    }

    private getValidator() {
        return (control) => {
            let promiseResolver;
            let p = new Promise((resolve) => {
                promiseResolver = resolve;
            });
            this.validate({ control: control, promiseResolver: promiseResolver });
            return p;
        };
    }

    static debounce(validator: (control: Control) => any, debounceTime = 400) {
        const asyncValidator = new this(validator, debounceTime);
        return asyncValidator.getValidator();
    }
}