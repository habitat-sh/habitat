A library allows arbitrary Ruby code to be included in a cookbook. The
most common use for libraries is to write helpers that are used
throughout recipes and custom resources. A library file is a Ruby file
that is located within a cookbook's `/libraries` directory. Because a
library is built using Ruby, anything that can be done with Ruby can be
done in a library file, including advanced functionality such as
extending built-in Chef classes.