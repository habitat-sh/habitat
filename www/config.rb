require 'slim'

set :markdown_engine, :redcarpet
set :markdown, fenced_code_blocks: true, tables: true, no_intra_emphasis: true, with_toc_data: true

require 'lib/lexer_habitat_studio'
activate :vegas

###
# Page options, layouts, aliases and proxies
###

# Per-page layout changes:
#
# With no layout
page '/*.xml', layout: false
page '/*.json', layout: false
page '/*.txt', layout: false

# With alternative layout
page 'about/*', layout: :sidebar, locals: { sidebar_layout: 'about' }
page 'docs/*', layout: :sidebar, locals: { sidebar_layout: 'docs' }
page 'legal/*', layout: :sidebar, locals: { sidebar_layout: 'legal' }
page '/demo/packaging-system/steps/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'demo_packaging_system'}
page '/demo/build-system/steps/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'demo_build_system'}
page '/demo/process-supervisor/steps/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'demo_process_supervisor'}
page '/demo/windows/steps/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'demo_windows'}
page 'get-started/*', layout: :get_started

activate :blog do |blog|
  blog.prefix = 'blog'
  blog.layout = 'layouts/blog_post'
  blog.permalink = '{year}/{month}/{title}.html'
  blog.default_extension = '.md'
  blog.summary_separator = /READMORE/
  blog.summary_length = 250
  blog.paginate = true
  blog.per_page = 10
  blog.page_link = 'page/{num}'
  blog.taglink = ':tag.html'
  blog.tag_template = 'blog/tag.html'
  blog.calendar_template = 'blog/calendar.html'
end

###
# Helpers
###

# Methods defined in the helpers block are available in templates
require 'lib/sidebar_helpers'
require 'lib/blog_helpers'
helpers SidebarHelpers
helpers BlogHelpers

helpers do
  def layout_class
    layout = current_page.options.fetch(:layout, nil)
    if layout == :sidebar
      'has-sidebar'
    elsif layout == :try
      'try-hab'
    elsif layout == :blog_post
      'blogs'
    elsif layout == :blog_index
      'has-sidebar'
    elsif layout == :tutorials
      'tutorials'
    else
      ''
    end
  end

  def path_starts_with?(path)
    current_page.path.start_with?(path)
  end

  def github_app_url
    ENV['GITHUB_APP_URL'] || 'https://github.com/apps/habitat-builder'
  end

  def builder_web_url
    ENV['BUILDER_WEB_URL'] || 'https://bldr.habitat.sh'
  end

  def github_www_source_url
    ENV['GITHUB_WWW_SOURCE_URL'] || 'https://github.com/habitat-sh/habitat/tree/master/www/source'
  end

  def render_markdown(text)
    Kramdown::Document.new(text).to_html
  end
end

activate :sprockets

configure :development do

  # Reload the browser automatically whenever files change
  activate :livereload
end

configure :build do

  # Asset hash to defeat caching between builds
  activate :asset_hash, :ignore => [/habitat-social.jpg/]
end

activate :autoprefixer
activate :directory_indexes

set :trailing_slash, false

activate :s3_sync do |s3_sync|
  s3_sync.path_style = false
  s3_sync.region = ENV['AWS_DEFAULT_REGION']
end

###
# Redirects
###
redirect 'blog/index.html', to: "http://blog.chef.io"
redirect 'about/index.html', to: '/about/announcement/'
redirect 'docs/build-packages-overview/index.html', to: '/docs/developing-packages#plan-builds/'
redirect 'docs/get-habitat/index.html', to: '/learn/'
redirect 'docs/try/index.html', to: '/docs/install-habitat/'
redirect 'download/index.html', to: '/docs/install-habitat/'
redirect 'downloads/index.html', to: '/docs/install-habitat/'
redirect 'try/index.html', to: '/learn/'
redirect 'try/2/index.html', to: '/learn/'
redirect 'try/3/index.html', to: '/learn/'
redirect 'try/4/index.html', to: '/learn/'
redirect 'try/5/index.html', to: '/learn/'
redirect 'try/6/index.html', to: '/learn/'
redirect 'try/7/index.html', to: '/learn/'
redirect 'try/8/index.html', to: '/learn/'
redirect 'try/9/index.html', to: '/learn/'
redirect 'try/10/index.html', to: '/learn/'
redirect 'tutorials/index.html', to: '/learn/'
redirect 'tutorials/download/index.html', to: '/docs/install-habitat/'
redirect 'tutorials/download/configure-workstation/index.html', to: '/docs/install-habitat/#configure-workstation'
redirect 'tutorials/getting-started/linux/add-hooks/index.html', to: '/learn/'
redirect 'tutorials/getting-started/linux/basic-concepts/index.html', to: '/learn/'
redirect 'tutorials/getting-started/linux/create-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/linux/configure-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/linux/process-build/index.html', to: '/learn/'
redirect 'tutorials/getting-started/linux/setup-environment/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/add-hooks/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/basic-concepts/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/create-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/configure-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/process-build/index.html', to: '/learn/'
redirect 'tutorials/getting-started/mac/setup-environment/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/add-hooks/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/basic-concepts/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/create-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/configure-plan/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/process-build/index.html', to: '/learn/'
redirect 'tutorials/getting-started/windows/setup-environment/index.html', to: '/learn/'
redirect 'tutorials/sample-app/basic-concepts/index.html', to: '/tutorials/sample-app/'
redirect 'docs/overview/index.html', to: '/docs/'
redirect 'docs/create-packages-overview/index.html', to: '/docs/developing-packages/'
redirect 'docs/create-plans/index.html', to: '/docs/developing-packages/#write-plans'
redirect 'docs/create-packages-configure/index.html', to: '/docs/developing-packages#add-configuration'
redirect 'docs/create-packages-build/index.html', to: '/docs/developing-packages#plan-builds'
redirect 'docs/create-packages-debugging/index.html', to: '/docs/developing-packages#debug-builds'
redirect 'docs/create-packages-binary-only/index.html', to: '/docs/best-practices#binary-wrapper'
redirect 'docs/run-packages-overview/index.html', to: '/docs/using-habitat#using-packages'
redirect 'docs/run-packages-service-groups/index.html', to: '/docs/using-habitat#service-groups'
redirect 'docs/run-packages-topologies/index.html', to: '/docs/using-habitat#topologies'
redirect 'docs/run-packages-apply-config-updates/index.html', to: '/docs/using-habitat#config-updates'
redirect 'docs/run-packages-upload-files/index.html', to: '/docs/using-habitat#file-uploads'
redirect 'docs/run-packages-security/index.html', to: '/docs/using-habitat#using-encryption'
redirect 'docs/run-packages-binding/index.html', to: '/docs/developing-packages#pkg-binds'
redirect 'docs/run-packages-update-strategy/index.html', to: '/docs/using-habitat#using-updates'
redirect 'docs/run-packages-multiple-services/index.html', to: '/docs/using-habitat#using-packages'
redirect 'docs/run-packages-export/index.html', to: '/docs/developing-packages#pkg-binds'
redirect 'docs/run-packages-monitoring/index.html', to: '/docs/using-habitat#monitor-services-through-the-http-api'
redirect 'docs/share-packages-overview/index.html', to: '/docs/developing-packages#sharing-pkgs'
redirect 'docs/continuous-deployment-overview/index.html', to: '/docs/'
redirect 'docs/container-orchestration/index.html', to: '/docs/best-practices#container-orchestration'
redirect 'docs/container-orchestration-ecs/index.html', to: '/docs/best-practices#ecs-and-habitat'
redirect 'docs/container-orchestration-mesos/index.html', to: '/docs/best-practices#mesos-dcos'
redirect 'docs/container-orchestration-kubernetes/index.html', to: '/docs/best-practices#kubernetes'
redirect 'docs/internals-overview/index.html', to: '/docs/internals/'
redirect 'docs/internals-supervisor/index.html', to: '/docs/internals/#supervisor-internals'
redirect 'docs/internals-leader-election/index.html', to: '/docs/internals#election-internals'
redirect 'docs/internals-crypto/index.html', to: '/docs/internals#crypto-internals'
redirect 'docs/internals-bootstrapping/index.html', to: '/docs/internals#bootstrap-internals'
redirect 'docs/reference/habitat-cli/index.html', to: '/docs/habitat-cli'
redirect 'docs/reference/plan-syntax/index.html', to: '/docs/reference'
redirect 'docs/reference/basic-settings/index.html', to: '/docs/reference/#plan-settings'
redirect 'docs/reference/callbacks/index.html', to: '/docs/reference/#reference-callbacks'
redirect 'docs/reference/build-variables/index.html', to: '/docs/reference/#plan-variables'
redirect 'docs/reference/hooks/index.html', to: '/docs/reference/#reference-hooks'
redirect 'docs/reference/runtime-settings/index.html', to: '/docs/reference/#template-data'
redirect 'docs/reference/utility-functions/index.html', to: '/docs/reference/#utility-functions'
redirect 'docs/reference/environment-vars/index.html', to: '/docs/reference/#environment-variables'
redirect 'docs/reference/package-contents/index.html', to: '/docs/reference/#package-contents'
redirect 'docs/reference/log-keys/index.html', to: '/docs/reference/#sup-log-keys'
redirect 'docs/reference/habitat-infographics/index.html', to: '/docs/diagrams'
redirect 'docs/contribute-help-build/index.html', to: '/docs/contribute'
redirect 'docs/concepts-scaffolding/index.html', to: '/docs/glossary/#glossary-scaffolding'
redirect 'docs/concepts-supervisor/index.html', to: '/docs/glossary/#glossary-supervisor'
redirect 'docs/concepts-plans/index.html', to: '/docs/glossary/#glossary-plan'
redirect 'docs/concepts-packages/index.html', to: '/docs/glossary/#glossary-artifacts'
redirect 'docs/concepts-keys/index.html', to: '/docs/glossary/#glossary-keys'
redirect 'docs/concepts-studio/index.html', to: '/docs/glossary/#glossary-studio'
redirect 'docs/concepts-services/index.html', to: '/docs/glossary/#glossary-services'
redirect 'docs/concepts-depot/index.html', to: '/docs/glossary/#glossary-builder'
redirect 'docs/concepts-overview/index.html', to: '/docs/glossary/'
redirect 'get-started/index.html', to: '/learn/'
redirect 'kubernetes/index.html', to: '/get-started/kubernetes/'
redirect 'demo/index.html', to: '/learn/'
redirect 'demo/packaging-system/index.html', to: '/demo/packaging-system/steps/1'
redirect 'demo/build-system/index.html', to: '/demo/build-system/steps/1'
redirect 'demo/process-supervisor/index.html', to: '/demo/process-supervisor/steps/1'
redirect 'legal/index.html', to: '/legal/licensing'
redirect 'cloudfoundry/index.html', to: '/get-started/cloudfoundry'
redirect 'pricing/index.html', to: '/enterprise'
