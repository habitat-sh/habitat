import { Pipe, PipeTransform } from "@angular/core";
import { GitHubRepo } from "../github-repos/shared/github-repos.model";

@Pipe({
    name: "habGitHubRepoFilter",
    pure: false
})
export class RepoFilterPipe implements PipeTransform {
    transform(items: GitHubRepo[], filter: GitHubRepo, filterParam: string): any {
        if (!items || !filter[filterParam]) {
            return items;
        }
        // filter items array, items which match and return true will be kept, false will be filtered out
        return items.filter((item) => {
            return item.get(filterParam).toLowerCase().indexOf(filter[filterParam].toLowerCase()) > -1;
        });
    }
}