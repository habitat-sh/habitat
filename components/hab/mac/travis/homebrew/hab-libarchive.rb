# Forked from https://raw.githubusercontent.com/Homebrew/homebrew-core/master/Formula/libarchive.rb
# On 2016-05-25

class HabLibarchive < Formula
  desc "Multi-format archive and compression library"
  homepage "http://www.libarchive.org"
  url "http://www.libarchive.org/downloads/libarchive-3.2.0.tar.gz"
  mirror "https://github.com/libarchive/libarchive/archive/v3.2.0.tar.gz"
  sha256 "7bce45fd71ff01dc20d19edd78322d4965583d81b8bed8e26cacb65d6f5baa87"

  keg_only :provided_by_osx

  depends_on "xz" => :recommended
  depends_on "lz4" => :optional
  depends_on "lzop" => :optional

  def install
    system "./configure",
           "--prefix=#{prefix}",
           "--without-lzo2",    # Use lzop binary instead of lzo2 due to GPL
           "--without-nettle",  # xar hashing option but GPLv3
           "--without-xml2",    # xar hashing option but tricky dependencies
           "--without-openssl", # mtree hashing now possible without OpenSSL
           "--with-expat",      # best xar hashing option
           "--with-libiconv-prefix=/usr/local/opt/hab-libiconv",
           "--enable-shared=no"

    system "make", "install"

    # Just as apple does it.
    ln_s bin/"bsdtar", bin/"tar"
    ln_s bin/"bsdcpio", bin/"cpio"
    ln_s man1/"bsdtar.1", man1/"tar.1"
    ln_s man1/"bsdcpio.1", man1/"cpio.1"
  end

  test do
    (testpath/"test").write("test")
    system bin/"bsdtar", "-czvf", "test.tar.gz", "test"
    assert_match /test/, shell_output("#{bin}/bsdtar -xOzf test.tar.gz")
  end
end
