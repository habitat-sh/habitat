resource "aws_volume_attachment" "api" {
  device_name = "/dev/xvdf"
  volume_id   = "${var.api_ebs_volume_id}"
  instance_id = "${aws_instance.api.id}"
}

resource "aws_volume_attachment" "database" {
  device_name = "/dev/xvdf"
  volume_id   = "${var.database_ebs_volume_id}"
  instance_id = "${aws_instance.datastore.id}"
}
