////////////////////////////////
// Front-end Instances

resource "aws_instance" "api" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.gateway.id}",
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
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-api --group ${var.env} --bind router:builder-router.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}",
      "sudo hab svc load core/builder-api-proxy --group ${var.env} --bind http:builder-api.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}",
    ]
  }

  tags {
    Name          = "builder-api-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "admin" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.gateway.id}",
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
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-admin --group ${var.env} --bind router:builder-router.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}",
      "sudo hab svc load core/builder-admin-proxy --group ${var.env} --bind http:builder-admin.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}",
    ]
  }

  tags {
    Name          = "builder-admin-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

////////////////////////////////
// Back-end Instances

resource "aws_instance" "datastore" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.datastore.id}",
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
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup_seed.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-datastore --group ${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-datastore-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "jobsrv" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.datastore_client.id}",
    "${aws_security_group.jobsrv.id}",
    "${aws_security_group.service.id}",
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
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-jobsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-jobsrv-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "originsrv" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.datastore_client.id}",
    "${aws_security_group.service.id}",
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
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-originsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-originsrv-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "router" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
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

  ebs_block_device {
    device_name = "/dev/xvdf"
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup_permanent.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-router --group ${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-router-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "scheduler" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.datastore_client.id}",
    "${aws_security_group.service.id}",
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
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-scheduler --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --bind depot:builder-api.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-scheduler-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "sessionsrv" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "t2.medium"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.datastore_client.id}",
    "${aws_security_group.service.id}",
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
    volume_size = 100
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-sessionsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}"
    ]
  }

  tags {
    Name          = "builder-sessionsrv-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "aws_instance" "worker" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "c4.2xlarge"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = "${var.jobsrv_worker_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${aws_security_group.jobsrv_client.id}",
    "${aws_security_group.worker.id}",
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
    volume_size = 250
    volume_type = "gp2"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/filesystem.sh",
      "${path.module}/scripts/bootstrap.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.hab_sup.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-worker --group ${var.env} --bind jobsrv:builder-jobsrv.${var.env} --bind depot:builder-api.${var.env} --strategy at-once --url ${var.depot_url} --channel ${var.release_channel}",
    ]
  }

  tags {
    Name          = "builder-worker-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

////////////////////////////////
// Template Files

data "template_file" "hab_sup" {
  template = "${file("${path.module}/templates/hab-sup.service")}"

  vars {
    flags               = "--auto-update --channel ${var.release_channel} --events hab-eventsrv.default --listen-gossip 0.0.0.0:${var.gossip_listen_port} --listen-http 0.0.0.0:${var.http_listen_port}"
    gossip_listen_port  = "${var.gossip_listen_port}"
    peer_ip             = "${aws_instance.datastore.0.private_ip}"
    log_level           = "${var.log_level}"
  }
}

data "template_file" "hab_sup_permanent" {
  template = "${file("${path.module}/templates/hab-sup.service")}"

  vars {
    flags               = "--auto-update --channel ${var.release_channel} --events hab-eventsrv.default --listen-gossip 0.0.0.0:${var.gossip_listen_port} --listen-http 0.0.0.0:${var.http_listen_port} --permanent-peer"
    gossip_listen_port  = "${var.gossip_listen_port}"
    peer_ip             = "${aws_instance.datastore.0.private_ip}"
    log_level           = "${var.log_level}"
  }
}

data "template_file" "hab_sup_seed" {
  template = "${file("${path.module}/templates/hab-sup.service")}"

  vars {
    flags               = "--auto-update --channel ${var.release_channel} --events hab-eventsrv.default --listen-gossip 0.0.0.0:${var.gossip_listen_port} --listen-http 0.0.0.0:${var.http_listen_port} --permanent-peer"
    gossip_listen_port  = "${var.gossip_listen_port}"
    peer_ip             = "127.0.0.1"
    log_level           = "${var.log_level}"
  }
}
