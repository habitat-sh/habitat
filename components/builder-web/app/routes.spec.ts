import { ExplorePageComponent } from "./explore-page/explore-page.component";
import { OriginCreatePageComponent } from "./origin-create-page/OriginCreatePageComponent";
import { OriginPageComponent } from "./origin-page/OriginPageComponent";
import { OriginsPageComponent } from "./origins-page/OriginsPageComponent";
import { OrganizationCreatePageComponent } from "./organization-create-page/OrganizationCreatePageComponent";
import { OrganizationsPageComponent } from "./organizations-page/OrganizationsPageComponent";
import { PackagePageComponent } from "./package-page/PackagePageComponent";
import { PackagesPageComponent } from "./packages-page/PackagesPageComponent";
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
    it("redirects to /pkgs/core", () => {
      let r = route("");
      expect(r.redirectTo).toBe("/pkgs/core");
      expect(r.pathMatch).toBe("full");
    });
  });

  describe("/explore", () => {
    it("routes to ExplorePageComponent", () => {
      let r = route("explore");
      expect(r.component).toBe(ExplorePageComponent);
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

  describe("/orgs", () => {
    it("routes to OrganizationsPageComponent", () => {
      let r = route("orgs");
      expect(r.component).toBe(OrganizationsPageComponent);
    });
  });

  describe("/orgs/create", () => {
    it("routes to OrganizationCreatePageComponent", () => {
      let r = route("orgs/create");
      expect(r.component).toBe(OrganizationCreatePageComponent);
    });
  });

  describe("/pkgs", () => {
    it("routes to PackagesPageComponent", () => {
      let r = route("pkgs");
      expect(r.component).toBe(PackagesPageComponent);
    });
  });

  describe("/pkgs/*/:name", () => {
    it("routes to PackagesPageComponent", () => {
      let r = route("pkgs/*/:name");
      expect(r.component).toBe(PackagesPageComponent);
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
});
