resource "aws_instance" "router" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"                // JW TODO: switch to private subnet after VPN is ready
  count         = "${var.router_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.router.id}",
  ]

  connection {
    // JW TODO: switch to private ip after VPN is ready
    host        = "${self.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  # JW TODO: Bake AMIs with updated habitat on them instead of bootstrapping
  provisioner "remote-exec" {
    script = "${path.module}/scripts/bootstrap.sh"
  }

  provisioner "file" {
    source      = "${path.module}/files/hab-director.service"
    destination = "/home/ubuntu/hab-director.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-director.service /etc/systemd/system/hab-director.service",
      "sudo mkdir -p /hab/etc/director",
      "cat <<BODY > /tmp/director-config.toml",
      "${data.template_file.router_director.rendered}",
      "BODY",
      "sudo mv /tmp/director-config.toml /hab/etc/director/config.toml",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-director",
      "sudo systemctl enable hab-director",
    ]
  }

  tags {
    Name          = "builder-router-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}

data "template_file" "router_director" {
  template = "${file("${path.module}/templates/router-director.toml")}"

  vars {
    env = "${var.env}"
  }
}

resource "aws_security_group" "router_gateway" {
  name   = "builder-router-gateway-${var.env}"
  vpc_id = "${var.aws_vpc_id}"

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}

resource "aws_security_group" "router_service" {
  name   = "builder-router-service-${var.env}"
  vpc_id = "${var.aws_vpc_id}"

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
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
      "${aws_security_group.router_gateway.id}",
    ]
  }

  ingress {
    from_port = 5562
    to_port   = 5563
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.router_service.id}",
    ]
  }

  // JW TODO: do we need public internet access?
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
  }
}
