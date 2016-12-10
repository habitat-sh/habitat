resource "aws_instance" "monolith" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"
  count         = "${var.monolith_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.builder_api.id}",
    "${aws_security_group.router_gateway.id}",
    "${aws_security_group.admin_gateway.id}",
  ]

  connection {
    // JW TODO: switch to private ip after VPN is ready
    host        = "${self.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  ebs_block_device {
    device_name = "/dev/xvdf"
    volume_size = 1500
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkfs.ext4 /dev/xvdf",
      "sudo mount /dev/xvdf /mnt",
      "echo '/dev/xvdf /hab     ext4   defaults 0 0' | sudo tee -a /etc/fstab",
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
      "${data.template_file.monolith_director.rendered}",
      "BODY",
      "sudo mv /tmp/director-config.toml /hab/etc/director/config.toml",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-director",
      "sudo systemctl enable hab-director",
    ]
  }

  tags {
    Name          = "builder-monolith-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
  }
}

data "template_file" "monolith_director" {
  template = "${file("${path.module}/templates/monolith-director.toml")}"

  vars {
    env = "${var.env}"
  }
}
