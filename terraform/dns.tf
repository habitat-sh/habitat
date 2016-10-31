resource "aws_route53_record" "app" {
  zone_id = "${var.dns_zone_id}"
  name    = "app-${var.env}"
  type    = "CNAME"
  ttl     = "300"
  records = ["${aws_elb.builder_api.dns_name}"]
}

resource "aws_route53_record" "willem" {
  zone_id = "${var.dns_zone_id}"
  name    = "willem-${var.env}"
  type    = "CNAME"
  ttl     = "300"
  records = ["${aws_elb.builder_api.dns_name}"]
}
