habitatConfig({
    habitat_api_url: "{{cfg.web.app_url}}",
    community_url: "{{cfg.web.community_url}}",
    docs_url: "{{cfg.web.docs_url}}",
    environment: "{{cfg.web.environment}}",
    friends_only: {{cfg.web.friends_only}},
    github_client_id: "{{cfg.github.client_id}}",
    source_code_url: "{{cfg.web.source_code_url}}",
    tutorials_url: "{{cfg.web.tutorials_url}}",
    version: "{{pkg.ident}}",
    www_url: "{{cfg.web.www_url}}",
});
