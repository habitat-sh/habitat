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
    layout == :sidebar ? 'has-sidebar' : ''
  end
end

# Build-specific configuration
configure :build do
  # Minify CSS on build
  # activate :minify_css

  # Minify Javascript on build
  # activate :minify_javascript
end

activate :autoprefixer
activate :directory_indexes

set :markdown_engine, :kramdown
set :markdown, :coderay_line_numbers => :table
