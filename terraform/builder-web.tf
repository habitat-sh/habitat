resource "aws_security_group" "habitat_web" {
  name        = "Habitat Web"
  description = "For traffic to HTTP(S) services"
  vpc_id      = "${aws_vpc.habitat_internal.id}"

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name           = "Habitat Web"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_instance" "habitat_builder_web" {
  ami                         = "${var.rhel7_ami}"
  associate_public_ip_address = true
  instance_type               = "t2.medium"
  key_name                    = "smith@nlsmith.com"
  subnet_id                   = "${aws_subnet.habitat.id}"
  vpc_security_group_ids      = ["${aws_security_group.habitat_web.id}"]

  provisioner "file" {
    source      = "components/studio/install.sh"
    destination = "/home/ec2-user/hab-studio-install.sh"

    connection {
      user = "ec2-user"
    }
  }

  provisioner "file" {
    source      = "terraform/scripts/"
    destination = "/home/ec2-user"

    connection {
      user = "ec2-user"
    }
  }

  provisioner "file" {
    source      = "terraform/config/"
    destination = "/home/ec2-user"

    connection {
      user = "ec2-user"
    }
  }

  provisioner "remote-exec" {
    inline = ["sudo bash /home/ec2-user/bootstrap.sh"]

    connection {
      user = "ec2-user"
    }
  }

  tags {
    Name           = "Habitat Builder Web 0"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_elb" "habitat_web" {
  name            = "habitat-web"
  security_groups = ["${aws_security_group.habitat_elb.id}"]
  subnets         = ["${aws_subnet.habitat.id}"]
  instances       = ["${aws_instance.habitat_builder_web.id}"]

  # Don't let this be destroyed since we need the CNAME
  lifecycle {
    prevent_destroy = true
  }

  listener {
    instance_port     = 80
    instance_protocol = "HTTP"
    lb_port           = 80
    lb_protocol       = "HTTP"
  }

  listener {
    instance_port      = 80
    instance_protocol  = "HTTP"
    lb_port            = 443
    lb_protocol        = "HTTPS"
    ssl_certificate_id = "arn:aws:iam::862552916454:server-certificate/habitat-sh"
  }

  tags {
    Name           = "Habitat Web"
    X-Dept         = "eng"
    X-Contact      = "smith@chef.io"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}
