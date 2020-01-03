Describe "filesystem with name >1024 characters" {
  sudo --preserve-env hab pkg install core/e2fsprogs --channel stable
  # Maximum directory name length is 255 characters so we need to create
  # a nested set of directories to have a mount point with > 1024 characters. 
  $tmpdir = New-TemporaryDirectory
  $directory="a"*100
  $mnt_path="/mnt/$("$directory/"*10)"

  cd "$tmpdir"

  # Create a tiny filesystem and mount it as a loopback device before we 
  # create our studio. It is important that this happens before we create the 
  # studio so that it appears first in /proc/mounts. The specific bug this is 
  # intended to detect (https://github.com/habitat-sh/habitat/issues/6591) 
  # won't be triggered if the studio mount entries are first. 
  dd if=/dev/zero of=empty-fs.img bs=10M count=1 
  hab pkg exec core/e2fsprogs mkfs.ext4 empty-fs.img
  sudo mkdir -p "$mnt_path"
  sudo mount -o loop "$tmpdir/empty-fs.img" $mnt_path
  mkdir studio 
  cd studio
  hab origin key generate "$HAB_ORIGIN"

  It "Removes the studio without errors" {
    hab studio new
    mount
    hab studio rm
    $LASTEXITCODE | Should -Be 0
  }
}
