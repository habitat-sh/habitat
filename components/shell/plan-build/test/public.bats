load '../bin/public'

@test "command exists" {
  run exists 'bats'
  [ "$status" -eq 0 ]
}

@test "command non-existent" {
  run exists 'no-bats'
  [ "$status" -eq 1 ]
}

@test "trim spaces" {
  result=$(trim '  example  ')
  [ "$result" == 'example' ]
}

@test "trim tabs" {
  result=$(trim '	example	')
  [ "$result" == 'example' ]
}

@test "trim empty" {
  result=$(trim)
  [ "$result" == '' ]
}

@test "trim no change" {
  result=$(trim 'example')
  [ "$result" == 'example' ]
}

@test "join by" {
  result=$(join_by ':' 'a' 'b' 'c')
  [ "$result" == 'a:b:c' ]
}

@test "join by array" {
  a=('a' 'b' 'c')
  result=$(join_by ':' ${a[@]})
  [ "$result" == 'a:b:c' ]
}

@test "join by empty" {
  result=$(join_by ':')
  [ "$result" == '' ]
}

@test "join by single" {
  result=$(join_by ':' 'a')
  [ "$result" == 'a' ]
}
