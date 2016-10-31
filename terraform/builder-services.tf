resource "aws_instance" "services" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"                // JW TODO: switch to private subnet after VPN is ready
  count         = "${var.service_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.service.id}",
    "${aws_security_group.router_service.id}",
  ]

  connection {
    // JW TODO: switch to private ip after VPN is ready
    host        = "${self.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  ebs_block_device {
    device_name = "/dev/xvdb"
    volume_size = 100
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /mnt/hab",
      "sudo ln -s /mnt/hab /hab",
    ]
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
      "${data.template_file.services_director.rendered}",
      "BODY",
      "sudo mv /tmp/director-config.toml /hab/etc/director/config.toml",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-director",
      "sudo systemctl enable hab-director",
    ]
  }

  tags {
    Name          = "builder-service-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}

data "template_file" "services_director" {
  template = "${file("${path.module}/templates/services-director.toml")}"

  vars {
    env = "${var.env}"

    // peer_ip = "${aws_instance.router.0.private_ip}"
    peer_ip = "${aws_instance.monolith.0.private_ip}"
  }
}

resource "aws_security_group" "service" {
  name        = "builder-service-${var.env}"
  description = "Allow traffic to and from Habitat Builder service instance"
  vpc_id      = "${var.aws_vpc_id}"

  ingress {
    from_port = 5566
    to_port   = 5567
    protocol  = "tcp"

    security_groups = [
      "${aws_security_group.jobsrv_worker.id}",
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

resource "aws_security_group" "jobsrv_worker" {
  name        = "builder-jobsrv-worker-${var.env}"
  description = "Assign to worker nodes to enable connectivity with job server on a service node"
  vpc_id      = "${var.aws_vpc_id}"

  tags {
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}
