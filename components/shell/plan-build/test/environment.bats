load '../bin/public'
load '../bin/environment'

@test "fail on unrecognized environment" {
  run __fail_on_unrecognized_env '__fake_environment'
  [ "$status" -eq 1 ]
}

@test "don't fail on buildtime environment" {
  run __fail_on_unrecognized_env '__buildtime_environment'
  [ "$status" -eq 0 ]
}

@test "don't fail on runtime environment" {
  run __fail_on_unrecognized_env '__runtime_environment'
  [ "$status" -eq 0 ]
}

@test "fail on protected environment variable manipulation" {
  run __fail_on_protected_env_var_manipulation 'PATH'
  [ "$status" -eq 1 ]
}

@test "unprotected environment variable manipulation" {
  run __fail_on_protected_env_var_manipulation 'HABITAT'
  [ "$status" -eq 0 ]
}

@test "dedupe path" {
  result=$(dedupe_path 'a:b:c:b:a')
  [ "$result" == 'a:b:c' ]
}

@test "dedupe path with separator" {
  result=$(dedupe_path 'a;b;c;b;a' ';')
  [ "$result" == 'a;b;c' ]
}

@test "dedupe path no change" {
  result=$(dedupe_path 'a:b:c')
  [ "$result" == 'a:b:c' ]
}

@test "dedupe path single" {
  result=$(dedupe_path 'a')
  [ "$result" == 'a' ]
}
