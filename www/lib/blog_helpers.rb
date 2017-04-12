module BlogHelpers
  def author_data(author_name)
    cleaned = clean_name(author_name)
    unless data.author_bios.authors.include?(cleaned)
      fail "'#{cleaned}' is not a registered author."
    end

    data.public_send(:author_bios).authors["#{cleaned}"]
  end

  def clean_name(name)
    name.gsub(/\s/,'').downcase
  end
end
