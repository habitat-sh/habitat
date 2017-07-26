require 'slim'

activate :syntax
set :markdown_engine, :kramdown
set :markdown, coderay_line_numbers: :table

###
# Page options, layouts, aliases and proxies
###

# Per-page layout changes:
#
# With no layout
page '/*.xml', layout: false
page '/*.json', layout: false
page '/*.txt', layout: false
page '/blog/feed.xml', layout: false

# With alternative layout
page 'about/*', layout: :sidebar, locals: { sidebar_layout: 'about' }
page 'docs/*', layout: :sidebar, locals: { sidebar_layout: 'docs' }
page 'legal/*', layout: :sidebar, locals: { sidebar_layout: 'legal' }
page 'tutorials/index.html', layout: :tutorials
page 'tutorials/get-started/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'get_started' }
page 'tutorials/download/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'download' }
page 'tutorials/sample-app/linux/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'sample_app_linux' }
page 'tutorials/sample-app/windows/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'sample_app_windows' }
page 'tutorials/sample-app/mac/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'sample_app_mac' }
page 'tutorials/sample-app/*', layout: :tutorials_sidebar, locals: { sidebar_layout: 'sample_app' }
page '/blog/index.html', layout: :blog_index

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

  def builder_web_url
    ENV['BUILDER_WEB_URL'] || 'https://bldr.habitat.sh'
  end

  def render_markdown(text)
    Kramdown::Document.new(text).to_html
  end
end

configure :development do

  # Reload the browser automatically whenever files change
  activate :livereload
end

configure :build do

  # Asset hash to defeat caching between builds
  activate :asset_hash
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
redirect 'about/index.html', to: '/about/announcement/'
redirect 'docs/build-packages-overview.html', to: '/docs/create-packages-build/'
redirect 'docs/get-habitat.html', to: '/tutorials/download/'
redirect 'download/index.html', to: '/tutorials/download/'
redirect 'downloads/index.html', to: '/tutorials/download/'
redirect 'try/index.html', to: '/tutorials/get-started/demo/'
redirect 'try/index.html', to: '/tutorials/'
redirect 'try/2/index.html', to: '/tutorials/'
redirect 'try/3/index.html', to: '/tutorials/'
redirect 'try/4/index.html', to: '/tutorials/'
redirect 'try/5/index.html', to: '/tutorials/'
redirect 'try/6/index.html', to: '/tutorials/'
redirect 'try/7/index.html', to: '/tutorials/'
redirect 'try/8/index.html', to: '/tutorials/'
redirect 'try/9/index.html', to: '/tutorials/'
redirect 'try/10/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/add-hooks/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/basic-concepts/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/create-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/configure-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/process-build/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/linux/setup-environment/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/add-hooks/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/basic-concepts/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/create-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/configure-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/process-build/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/mac/setup-environment/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/add-hooks/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/basic-concepts/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/create-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/configure-plan/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/process-build/index.html', to: '/tutorials/'
redirect 'tutorials/getting-started/windows/setup-environment/index.html', to: '/tutorials/'
