resource "aws_security_group" "datastore" {
  name        = "builder-datastore-${var.env}"
  description = "Builder datastore instance"
  vpc_id      = "${var.aws_vpc_id}"

  ingress {
    from_port = 5432
    to_port   = 5432
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.datastore_client.id}",
    ]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "datastore_client" {
  name        = "builder-datastore-client-${var.env}"
  description = "Grants an instance client traffic to and from a datastore"
  vpc_id      = "${var.aws_vpc_id}"

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "gateway" {
  name        = "builder-gateway-${var.env}"
  description = "Habitat Builder Gateway"
  vpc_id = "${var.aws_vpc_id}"

  ingress {
    from_port = 80
    to_port   = 80
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.gateway_elb.id}",
    ]
  }

  ingress {
    from_port = 8081
    to_port   = 8081
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.gateway_elb.id}",
    ]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "gateway_elb" {
  name        = "builder-gateway-elb-${var.env}"
  description = "Habitat Builder Gateway Load Balancer"
  vpc_id      = "${var.aws_vpc_id}"

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
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "jobsrv" {
  name        = "builder-jobsrv-${var.env}"
  description = "Builder JobSrv instance"
  vpc_id      = "${var.aws_vpc_id}"

  ingress {
    from_port = 5566
    to_port   = 5568
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.jobsrv_client.id}",
    ]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "jobsrv_client" {
  name        = "builder-jobsrv-client-${var.env}"
  description = "Grants an instance client traffic to and from a JobSrv"
  vpc_id      = "${var.aws_vpc_id}"

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "router" {
  name        = "builder-router-${var.env}"
  description = "Allow traffic to and from Habitat Builder RouteSrv"
  vpc_id      = "${var.aws_vpc_id}"

  ingress {
    from_port = 5562
    to_port   = 5562
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.gateway.id}",
    ]
  }

  ingress {
    from_port = 5562
    to_port   = 5563
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.service.id}",
    ]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "service" {
  name   = "builder-service-${var.env}"
  vpc_id = "${var.aws_vpc_id}"

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "worker" {
  name   = "builder-worker-${var.env}"
  vpc_id = "${var.aws_vpc_id}"

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_security_group" "worker_studio" {
  name   = "builder-worker-studio-${var.env}"
  vpc_id = "${var.aws_vpc_id}"

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}
