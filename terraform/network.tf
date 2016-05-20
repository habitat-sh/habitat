provider "aws" {
  region = "${var.aws_region}"
}

resource "aws_vpc" "habitat_internal" {
  cidr_block = "10.0.0.0/16"

  tags {
    Name           = "Habitat Internal"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_security_group" "habitat_elb" {
  name        = "Habitat ELB"
  description = "Habitat Load Balancer"
  vpc_id      = "${aws_vpc.habitat_internal.id}"

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

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    Name           = "Habitat ELB"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_internet_gateway" "habitat" {
  vpc_id = "${aws_vpc.habitat_internal.id}"

  tags {
    Name           = "Habitat"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}

resource "aws_route" "habitat_internet_access" {
  route_table_id         = "${aws_vpc.habitat_internal.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = "${aws_internet_gateway.habitat.id}"
}

resource "aws_subnet" "habitat" {
  vpc_id                  = "${aws_vpc.habitat_internal.id}"
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags {
    Name           = "Habitat"
    X-Dept         = "eng"
    X-Contact      = "Nathan L Smith <smith@chef.io>"
    X-Production = true
    X-Environment  = "production"
    X-Application  = "habitat"
  }
}
