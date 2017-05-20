resource "aws_route53_record" "admin" {
  zone_id = "${var.dns_zone_id}"
  name    = "admin.${var.env}"
  type    = "CNAME"
  ttl     = 300
  records = ["${aws_elb.admin.dns_name}"]
}

resource "aws_route53_record" "api" {
  zone_id = "${var.dns_zone_id}"
  name    = "api-${var.env}"
  type    = "CNAME"
  ttl     = 300
  records = ["${aws_elb.api.dns_name}"]
}

resource "aws_route53_record" "app" {
  zone_id = "${var.dns_zone_id}"
  name    = "app-${var.env}"
  type    = "CNAME"
  ttl     = 300
  records = ["${aws_elb.api.dns_name}"]
}

resource "aws_route53_record" "bldr" {
  zone_id = "${var.dns_zone_id}"
  name    = "bldr-${var.env}"
  type    = "CNAME"
  ttl     = 300
  records = ["${aws_elb.api.dns_name}"]
}

resource "aws_route53_record" "willem" {
  zone_id = "${var.dns_zone_id}"
  name    = "willem-${var.env}"
  type    = "CNAME"
  ttl     = 300
  records = ["${aws_elb.api.dns_name}"]
}
