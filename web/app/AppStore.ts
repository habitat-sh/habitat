import {Injectable} from "angular2/core";
import {createStore} from "redux";
import {rootReducer} from "./rootReducer"

const appStore = createStore(rootReducer);

@Injectable()
export class AppStore {
  private store = appStore;

  getState() {
    return this.store.getState();
  }

  dispatch(action) {
    this.store.dispatch(action);
  }

  subscribe(listener: Function) {
    this.store.subscribe(() => listener(this.getState()));
  }
}
