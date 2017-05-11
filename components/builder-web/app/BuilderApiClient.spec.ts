import { BuilderApiClient } from "./BuilderApiClient";

describe("BuilderApiClient", () => {

  describe("acceptOriginInvitation", () => {

    describe("on success", () => {
      let myObj;

      beforeEach(() => {
        myObj = { myCallback: () => {} };
        spyOn(myObj, "myCallback");
        spyOn(window, "fetch").and.callFake(() => {
          return Promise.resolve({ ok: true });
        });
      });

      it("resolves true", (done) => {
        new BuilderApiClient("mytoken")
          .acceptOriginInvitation("123", "myorigin")
          .then(myObj.myCallback)
          .then(() => {
            expect(myObj.myCallback).toHaveBeenCalledWith(true);
            done();
          });
      });
    });

    describe("on failure", () => {
      let myObj;

      beforeEach(() => {
        myObj = { myCallback: () => {} };
        spyOn(myObj, "myCallback");
        spyOn(window, "fetch").and.callFake(() => {
          return Promise.resolve({ ok: false });
        });
      });

      it("resolves with an instance of Error", (done) => {
        new BuilderApiClient("some-token")
          .acceptOriginInvitation("123", "myorigin")
          .catch(myObj.myCallback)
          .then(() => {
            expect(myObj.myCallback).toHaveBeenCalledWith(jasmine.any(Error));
            done();
          });
      });
    });
  });
});
