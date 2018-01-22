class HabRq < Formula
  desc "Record Query - A tool for doing record analysis and transformation"
  homepage "https://github.com/dflemstr/rq"
  url "https://github.com/dflemstr/rq/releases/download/v0.9.2/rq-x86_64-apple-darwin"
  sha256 "fbc9347d83ee575c10251ad2fff9c31a78c42d4cedc14bbb9be72739ed619496"

  def install
    mv "rq-x86_64-apple-darwin", "rq", verbose: true
    bin.install "rq"
  end

  test do
    system "#{bin}/rq", "--version"
  end
end
