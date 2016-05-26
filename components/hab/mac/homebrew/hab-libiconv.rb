# Forked from https://raw.githubusercontent.com/Homebrew/homebrew-dupes/master/libiconv.rb
# On 2016-05-25
#
class HabLibiconv < Formula
  desc "Conversion library"
  homepage "https://www.gnu.org/software/libiconv/"
  url "http://ftpmirror.gnu.org/libiconv/libiconv-1.14.tar.gz"
  mirror "https://ftp.gnu.org/gnu/libiconv/libiconv-1.14.tar.gz"
  sha256 "72b24ded17d687193c3366d0ebe7cde1e6b18f0df8c55438ac95be39e8a30613"

  bottle do
    sha256 "64d8a9383ba42ba3e41422bb8548ebc8f296f67fdda6e6d6a324f990b03c6db0" => :el_capitan
    sha256 "a0d9ff36269bc908fde4a039d2083152202055a2e053b6582ad2c9063c85ebc2" => :yosemite
    sha256 "456a816a94427c963fa3cb90257830aa33268f22443cf5a8a4cf1be3e3ed3bb9" => :mavericks
  end

  keg_only :provided_by_osx

  option :universal

  patch do
    url "https://raw.githubusercontent.com/Homebrew/patches/9be2793af/libiconv/patch-Makefile.devel"
    sha256 "ad9b6da1a82fc4de27d6f7086a3382993a0b16153bc8e8a23d7b5f9334ca0a42"
  end

  patch do
    url "https://raw.githubusercontent.com/Homebrew/patches/9be2793af/libiconv/patch-utf8mac.diff"
    sha256 "e8128732f22f63b5c656659786d2cf76f1450008f36bcf541285268c66cabeab"
  end

  patch :DATA

  def install
    ENV.universal_binary if build.universal?
    ENV.j1

    system "./configure", "--disable-debug",
                          "--disable-dependency-tracking",
                          "--prefix=#{prefix}",
                          "--enable-extra-encodings",
                          "--enable-static"
    system "make", "-f", "Makefile.devel", "CFLAGS=#{ENV.cflags}", "CC=#{ENV.cc}"
    system "make", "install"
  end
end


__END__
diff --git a/lib/flags.h b/lib/flags.h
index d7cda21..4cabcac 100644
--- a/lib/flags.h
+++ b/lib/flags.h
@@ -14,6 +14,7 @@
 
 #define ei_ascii_oflags (0)
 #define ei_utf8_oflags (HAVE_ACCENTS | HAVE_QUOTATION_MARKS | HAVE_HANGUL_JAMO)
+#define ei_utf8mac_oflags (HAVE_ACCENTS | HAVE_QUOTATION_MARKS | HAVE_HANGUL_JAMO)
 #define ei_ucs2_oflags (HAVE_ACCENTS | HAVE_QUOTATION_MARKS | HAVE_HANGUL_JAMO)
 #define ei_ucs2be_oflags (HAVE_ACCENTS | HAVE_QUOTATION_MARKS | HAVE_HANGUL_JAMO)
 #define ei_ucs2le_oflags (HAVE_ACCENTS | HAVE_QUOTATION_MARKS | HAVE_HANGUL_JAMO)
