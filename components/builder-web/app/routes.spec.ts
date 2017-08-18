import { BuildPageComponent } from "./build-page/build-page.component";
import { DashboardComponent } from "./dashboard/dashboard.component";
import { ExploreComponent } from "./explore/explore.component";
import { OriginCreatePageComponent } from "./origin-create-page/OriginCreatePageComponent";
import { OriginPageComponent } from "./origin-page/OriginPageComponent";
import { OriginsPageComponent } from "./origins-page/OriginsPageComponent";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackagePageComponent } from "./package-page/PackagePageComponent";
import { PackagesPageComponent } from "./packages-page/packages-page.component";
import { ProjectCreatePageComponent } from "./project-create-page/ProjectCreatePageComponent";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SCMReposPageComponent } from "./scm-repos-page/SCMReposPageComponent";
import { SignInPageComponent } from "./sign-in-page/SignInPageComponent";
import { ProjectSettingsPageComponent } from "./project-settings-page/ProjectSettingsPageComponent";
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

  describe("/explore", () => {
    it("routes to DashboardComponent", () => {
      let r = route("");
      expect(r.component).toBe(DashboardComponent);
    });
  });

  describe("/builds/:id", () => {
    it("routes to BuildPageComponent", () => {
      let r = route("builds/:id");
      expect(r.component).toBe(BuildPageComponent);
    });
  });

  describe("/origins", () => {
    it("routes to OriginsPageComponent", () => {
      let r = route("origins");
      expect(r.component).toBe(OriginsPageComponent);
    });
  });

  describe("/origins/create", () => {
    it("routes to OriginCreatePageComponent", () => {
      let r = route("origins/create");
      expect(r.component).toBe(OriginCreatePageComponent);
    });
  });

  describe("/origins/:origin", () => {
    it("routes to OriginPageComponent", () => {
      let r = route("origins/:origin");
      expect(r.component).toBe(OriginPageComponent);
    });
  });

  describe("/pkgs", () => {
    it("redirects to /pkgs/core", () => {
      let r = route("pkgs");
      expect(r.redirectTo).toBe("/pkgs/core");
    });
  });

  describe("/pkgs/:origin", () => {
    it("routes to PackagesPageComponent", () => {
      let r = route("pkgs/:origin");
      expect(r.component).toBe(PackagesPageComponent);
    });
  });

  describe("/pkgs/:origin/:name", () => {
    it("routes to PackagesPageComponent", () => {
      let r = route("pkgs/:origin/:name");
      expect(r.component).toBe(PackagesPageComponent);
    });
  });

  describe("/pkgs/:origin/:name/builds", () => {
    it("routes to PackageBuildsComponent", () => {
      let r = route("pkgs/:origin/:name/builds");
      expect(r.component).toBe(PackageBuildsComponent);
    });
  });

  describe("/pkgs/:origin/:name/latest", () => {
    it("routes to PackageLatestComponent", () => {
      let r = route("pkgs/:origin/:name/latest");
      expect(r.component).toBe(PackageLatestComponent);
    });
  });

  describe("/pkgs/:origin/:name/:version", () => {
    it("routes to PackagesPageComponent", () => {
      let r = route("pkgs/:origin/:name/:version");
      expect(r.component).toBe(PackagesPageComponent);
    });
  });

  describe("/pkgs/:origin/:name/:version/:release", () => {
    it("routes to PackagePageComponent", () => {
      let r = route("pkgs/:origin/:name/:version/:release");
      expect(r.component).toBe(PackagePageComponent);
    });
  });

  describe("/projects", () => {
    it("routes to ProjectsPageComponent", () => {
      let r = route("projects");
      expect(r.component).toBe(ProjectsPageComponent);
    });
  });

  describe("/projects/create", () => {
    it("routes to ProjectCreatePageComponent", () => {
      let r = route("projects/create");
      expect(r.component).toBe(ProjectCreatePageComponent);
    });
  });

  describe("/projects/:origin/:name", () => {
    it("routes to ProjectPageComponent", () => {
      let r = route("projects/:origin/:name");
      expect(r.component).toBe(ProjectPageComponent);
    });
  });

  describe("/projects/:origin/:name/settings", () => {
    it("routes to ProjectSettingsPageComponent", () => {
      let r = route("projects/:origin/:name/settings");
      expect(r.component).toBe(ProjectSettingsPageComponent);
    });
  });

  describe("/scm-repos", () => {
    it("routes to SCMReposPageComponent", () => {
      let r = route("scm-repos");
      expect(r.component).toBe(SCMReposPageComponent);
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
