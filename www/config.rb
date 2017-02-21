require 'slim'

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
page 'tutorials/getting-started/linux/*', layout: :sidebar, locals: { sidebar_layout: 'linux_tutorial' }
page 'tutorials/getting-started/windows/*', layout: :sidebar, locals: { sidebar_layout: 'windows_tutorial' }
page 'tutorials/*', layout: :sidebar, locals: { sidebar_layout: 'tutorials' }
page 'docs/*', layout: :sidebar, locals: { sidebar_layout: 'docs' }
page 'legal/*', layout: :sidebar, locals: { sidebar_layout: 'legal' }
page 'try/*', layout: :try

# Proxy pages (http://middlemanapp.com/basics/dynamic-pages/)
# proxy '/this-page-has-no-template.html', '/template-file.html', locals: {
#  which_fake_page: 'Rendering a fake page with a local variable' }

###
# Helpers
###

# Reload the browser automatically whenever files change
configure :development do
  activate :livereload
end

# Methods defined in the helpers block are available in templates
require 'lib/sidebar_helpers'
helpers SidebarHelpers

helpers do
  def layout_class
    layout = current_page.options.fetch(:layout, nil)
    if layout == :sidebar
      'has-sidebar'
    elsif layout == :try
      'try-hab'
    else
      ''
    end

  end
end

# Build-specific configuration
configure :build do
  # Minify CSS on build
  # activate :minify_css

  # Minify Javascript on build
  # activate :minify_javascript

  # Asset hash to defeat caching between builds
  activate :asset_hash
end

activate :autoprefixer
activate :directory_indexes
set :trailing_slash, false

activate :s3_sync do |s3_sync|
  s3_sync.path_style = false
  s3_sync.region = ENV["AWS_DEFAULT_REGION"]
end

set :markdown_engine, :kramdown
set :markdown, coderay_line_numbers: :table

###
# Redirects
###
# Temporarily changes the default redirect (from /about
  # index to 'Why Habitat' article) to a livestream page
# redirect 'about/index.html', to: 'about/why-habitat.html'
redirect 'about/index.html', to: 'about/announcement.html'
redirect 'docs/index.html', to: 'docs/overview.html'
redirect 'docs/build-packages-overview.html', to: 'docs/create-packages-build.html'
redirect 'tutorials/getting-started-overview.html', to: 'tutorials/getting-started/overview.html'
redirect 'tutorials/getting-started-basic-concepts.html', to: 'tutorials/getting-started/mac/basic-concepts.html'
redirect 'tutorials/getting-started-setup-environment.html', to: 'tutorials/getting-started/mac/setup-environment.html' 
redirect 'tutorials/getting-started-review-source-files.html', to: 'tutorials/getting-started/mac/create-plan.html'
redirect 'tutorials/getting-started-create-plan.html', to: 'tutorials/getting-started/mac/create-plan.html'
redirect 'tutorials/getting-started-add-hooks.html', to: 'tutorials/getting-started/mac/add-hooks.html'
redirect 'tutorials/getting-started-configure-plan.html', to: 'tutorials/getting-started/mac/configure-plan.html'
redirect 'tutorials/getting-started-process-build.html', to: 'tutorials/getting-started/mac/process-build.html'
