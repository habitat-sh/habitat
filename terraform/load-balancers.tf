resource "aws_elb" "admin" {
  name            = "builder-admin-gateway-${var.env}"
  security_groups = ["${aws_security_group.gateway_elb.id}"]
  subnets         = ["${var.public_subnet_id}"]
  instances       = ["${aws_instance.admin.*.id}"]

  listener {
    instance_port      = 8081
    instance_protocol  = "HTTP"
    lb_port            = 443
    lb_protocol        = "HTTPS"
    ssl_certificate_id = "${var.ssl_certificate_arn}"
  }

  health_check {
    healthy_threshold   = 2
    unhealthy_threshold = 5
    timeout             = 5
    target              = "HTTP:8081/v1/status"
    interval            = 30
  }

  tags {
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_elb" "api" {
  name            = "builder-api-gateway-${var.env}"
  security_groups = ["${aws_security_group.gateway_elb.id}"]
  subnets         = ["${var.public_subnet_id}"]
  instances       = ["${aws_instance.api.*.id}"]

  listener {
    instance_port      = 80
    instance_protocol  = "HTTP"
    lb_port            = 443
    lb_protocol        = "HTTPS"
    ssl_certificate_id = "${var.ssl_certificate_arn}"
  }

  health_check {
    healthy_threshold   = 2
    unhealthy_threshold = 5
    timeout             = 5
    target              = "HTTP:80/v1/status"
    interval            = 30
  }

  tags {
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

// We want this to be configured to have unsafe SSL Protocols and Ciphers turned off from the
// current default AWS ELB set (ELBSecurityPolicy-2015-05)
resource "aws_lb_ssl_negotiation_policy" "admin" {
  name          = "builder-admin"
  load_balancer = "${aws_elb.admin.id}"
  lb_port       = 443

  attribute {
    name  = "Protocol-TLSv1"
    value = "false"
  }

  attribute {
    name  = "Protocol-TLSv1.1"
    value = "false"
  }

  attribute {
    name  = "Protocol-TLSv1.2"
    value = "true"
  }

  attribute {
    name  = "Protocol-SSLv3"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "DHE-RSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-GCM-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES256-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "AES256-GCM-SHA384"
    value = "true"
  }

  attribute {
    name  = "AES256-SHA256"
    value = "true"
  }

  attribute {
    name  = "AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "DHE-DSS-AES128-SHA"
    value = "true"
  }

  attribute {
    name  = "DES-CBC3-SHA"
    value = "false"
  }
}

resource "aws_lb_ssl_negotiation_policy" "api" {
  name          = "builder-api"
  load_balancer = "${aws_elb.api.id}"
  lb_port       = 443

  attribute {
    name  = "Protocol-TLSv1"
    value = "false"
  }

  attribute {
    name  = "Protocol-TLSv1.1"
    value = "false"
  }

  attribute {
    name  = "Protocol-TLSv1.2"
    value = "true"
  }

  attribute {
    name  = "Protocol-SSLv3"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "ECDHE-RSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "DHE-RSA-AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-GCM-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES256-SHA384"
    value = "true"
  }

  attribute {
    name  = "ECDHE-RSA-AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "ECDHE-ECDSA-AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "AES128-GCM-SHA256"
    value = "false"
  }

  attribute {
    name  = "AES128-SHA256"
    value = "true"
  }

  attribute {
    name  = "AES128-SHA"
    value = "false"
  }

  attribute {
    name  = "AES256-GCM-SHA384"
    value = "true"
  }

  attribute {
    name  = "AES256-SHA256"
    value = "true"
  }

  attribute {
    name  = "AES256-SHA"
    value = "true"
  }

  attribute {
    name  = "DHE-DSS-AES128-SHA"
    value = "true"
  }

  attribute {
    name  = "DES-CBC3-SHA"
    value = "false"
  }
}
