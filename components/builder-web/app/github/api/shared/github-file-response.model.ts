import { GitHubFile } from "../../file/shared/github-file.model";

export interface GitHubFileResponse {
  total_count: number;
  incomplete_results: boolean;
  items?: Array<GitHubFile>;
}