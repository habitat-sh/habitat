// Queries for packages. `query(packages)` where `packages` is an array of
// object representing packages will return an object that has methods you can
// call for types of queries.
//
// All return Enumerable instances.

///<reference path="../vendor/typings/linq/linq.d.ts"/>

import * as Enumerable from "linq";

export default function query(packages: Array<any>) {
    let enumerable = Enumerable.from(packages);

    return {
        // The internal enumerable instance. You can use this to do "ad hoc" queries
        // instead of using the built-in ones.
        enumerable,

        // All of the packages for a given name, sorted by stars
        allForNameByStars(name: string) {
            return enumerable.
                where(`$.name === "${name}"`).
                groupBy("$.derivation").
                select(group => group.first()).
                orderByDescending("$.starCount");
        },

        // All of the packages, sorted by name, the most recent version.
        allMostRecent() {
            return enumerable.
                orderByDescending("$.release").
                groupBy("$.name").
                select(group => group.first()).
                orderBy("$.name");
        },

        // All of the packages, sorted by name, the most recent version, for a
        // derivation
        allMostRecentForDerivation(derivation: string) {
            return this.allMostRecent().
                where(`$.derivation === "${derivation}"`);
        },

        // Given a package, all of the releases for that version of the package
        allReleasesForPackageVersion(sourcePkg) {
            return enumerable.
                where(pkg => {
                    return pkg["name"] === sourcePkg["name"] &&
                        pkg["version"] === sourcePkg["version"];
                });
        },

        // Given a package, the most recent release of each version
        allVersionsForPackage(sourcePkg) {
            return enumerable.
                where(pkg => { return pkg["name"] === sourcePkg["name"]; }).
                groupBy("$.version").
                select(group => group.first()).
                orderByDescending("$.version"); // TODO: make this semver(ish) sorted
        },

        fromParams(params: Object = {}) {
            return enumerable.
                where(pkg => pkg["name"] === params["name"] &&
                    pkg["derivation"] === params["derivation"] &&
                    pkg["version"] === params["version"] &&
                    pkg["release"] === params["release"]);
        }
    };
}
