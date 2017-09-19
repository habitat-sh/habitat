import { Injectable } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { AppStore } from "../AppStore";
import { Origin } from "../records/Origin";

@Injectable()
export class OriginService {
  origin(originInRoute: string, currentOriginFromState) {
    if (currentOriginFromState.name === originInRoute) {
      return currentOriginFromState;
    } else {
      return Origin({ name: originInRoute });
    }
  }
}
