import { Injectable } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { AppStore } from "../AppStore";
import { OriginRecord } from "../records/origin-record";

@Injectable()
export class OriginService {
  origin(originInRoute: string, currentOriginFromState) {
    if (currentOriginFromState.name === originInRoute) {
      return currentOriginFromState;
    } else {
      return OriginRecord({ name: originInRoute });
    }
  }
}
