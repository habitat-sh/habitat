# The things we're doing in here have some hard-coded values that are based on

# what we alrady were running when this was written.

resource "aws_security_group" "habitat_api_elb" {
  name        = "Habitat API ELB"
  description = "Habitat API Load Balancer"
  vpc_id      = "vpc-3b678f5e"

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 9636
    to_port     = 9636
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 2375
    to_port     = 2375
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 2376
    to_port     = 2376
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 54856
    to_port     = 54856
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name           = "Habitat API ELB"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_elb" "habitat_api" {
  name            = "habitat-api"
  security_groups = ["${aws_security_group.habitat_api_elb.id}"]
  subnets         = ["subnet-5bbda12f"]
  instances       = ["i-5638008e"]

  # Don't let this be destroyed since we need the CNAME
  lifecycle {
    prevent_destroy = true
  }

  listener {
    instance_port     = 2375
    instance_protocol = "HTTP"
    lb_port           = 2375
    lb_protocol       = "HTTP"
  }

  listener {
    instance_port     = 2376
    instance_protocol = "HTTP"
    lb_port           = 2376
    lb_protocol       = "HTTP"
  }

  listener {
    instance_port     = 54856
    instance_protocol = "HTTP"
    lb_port           = 54856
    lb_protocol       = "HTTP"
  }

  listener {
    instance_port     = 9636
    instance_protocol = "HTTP"
    lb_port           = 9636
    lb_protocol       = "HTTP"
  }

  listener {
    instance_port      = 9636
    instance_protocol  = "HTTP"
    lb_port            = 80
    lb_protocol        = "HTTP"
  }

  listener {
    instance_port      = 9636
    instance_protocol  = "HTTP"
    lb_port            = 443
    lb_protocol        = "HTTPS"
    ssl_certificate_id = "arn:aws:iam::862552916454:server-certificate/habitat-sh"
  }

  health_check {
    healthy_threshold   = 10
    unhealthy_threshold = 2
    timeout             = 5
    target              = "HTTP:9636/v1/depot/pkgs/core"
    interval            = 30
  }

  tags {
    Name           = "Habitat API"
    X-Dept         = "eng"
    X-Contact      = "smith@chef.io"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}
