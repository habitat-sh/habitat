resource "aws_instance" "jobsrv_workers" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"                // JW TODO: switch to private subnet after VPN is ready
  count         = "${var.jobsrv_worker_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.worker.id}",
    "${aws_security_group.jobsrv_worker.id}",
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
      "${data.template_file.worker_director.rendered}",
      "BODY",
      "sudo mv /tmp/director-config.toml /hab/etc/director/config.toml",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-director",
      "sudo systemctl enable hab-director",
    ]
  }

  tags {
    Name          = "builder-worker-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}

data "template_file" "worker_director" {
  template = "${file("${path.module}/templates/worker-director.toml")}"

  vars {
    env = "${var.env}"

    // peer_ip = "${aws_instance.router.0.private_ip}"
    peer_ip = "${aws_instance.monolith.0.private_ip}"
  }
}

resource "aws_security_group" "worker" {
  name        = "builder-worker-${var.env}"
  description = "Basic Traffic rules for worker instances"
  vpc_id      = "${var.aws_vpc_id}"

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
