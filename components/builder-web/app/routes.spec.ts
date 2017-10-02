import { ExploreComponent } from "./explore/explore.component";
import { SignInPageComponent } from "./sign-in-page/sign-in-page.component";
import { routes } from "./routes";

describe("Routes", () => {

  function route(path) {
    return routes.find((r) => r.path === path);
  }

  describe("/", () => {
    it("routes to ExploreComponent", () => {
      let r = route("explore");
      expect(r.component).toBe(ExploreComponent);
    });
  });

  describe("/sign-in", () => {
    it("routes to SignInPageComponent", () => {
      let r = route("sign-in");
      expect(r.component).toBe(SignInPageComponent);
    });
  });

  describe("non-existent routes", () => {
    it("redirect to /pkgs/core", () => {
      let r = route("*");
      let lastRoute = routes[routes.length - 1];
      expect(r.redirectTo).toBe("/pkgs/core");
      expect(lastRoute).toBe(r);
    });
  });
});
