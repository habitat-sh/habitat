habitatConfig({
    // The URL for the Habitat API service (including the API version.) If
    // running the API services locally with `make bldr-run` or `make bldr-run-shell` from the root
    // of the habitat repo, this will be localhost (if running Docker for Mac or
    // Linux) or the result of `$(docker-machine ip default)` if using Docker
    // in a virtual Machine.
    habitat_api_url: "http://localhost:9636/v1",
    // The URL for community information
    community_url: "https://www.habitat.sh/community",
    // The URL for documentation
    docs_url: "https://www.habitat.sh/docs",
    // The environment in which we're running. If "production", enable
    // production mode
    environment: "production",
    // The URL for Habitat's source code
    // GitHub Client ID for OAuth2
    // The example is for builder-dev: https://github.com/settings/connections/applications/0c2f738a7d0bd300de10
    github_client_id: "0c2f738a7d0bd300de10",
    // The URL for GitHub
    github_url: "https://github.com"
    // The URL for the Habitat source code
    source_code_url: "https://github.com/habitat-sh/habitat",
    // The URL for tutorials
    tutorials_url: "https://www.habitat.sh/tutorials",
    // The URL for Slack
    slack_url: "http://slack.habitat.sh/",
    // The URL for YouTube
    youtube_url: "https://www.youtube.com/playlist?list=PL11cZfNdwNyOxlvI1Kq6ae8eVBl5S3IKk",
    // The URL for events
    events_url: "https://events.chef.io/events/categories/habitat/",
    // The URL for roadmap
    roadmap_url: "https://ext.prodpad.com/ext/roadmap/d2938aed0d0ad1dd62669583e108357efd53b3a6",
    // The URL for feature requests
    feature_requests_url: "https://portal.prodpad.com/24539",
    // The URL for forums
    forums_url: "https://forums.habitat.sh/",
    // The version of the software we're running. In production, this should
    // be automatically populated by Habitat
    version: "",
    // The main website URL
    www_url: "https://www.habitat.sh",
});
