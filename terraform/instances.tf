////////////////////////////////
// Front-end Instances

resource "aws_instance" "api" {
  ami           = "${lookup(var.aws_ami, var.aws_region)}"
  instance_type = "${var.instance_size_api}"
  key_name      = "${var.aws_key_pair}"
  subnet_id     = "${var.public_subnet_id}"
  count         = 1

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${var.events_sg}",
    "${aws_security_group.gateway.id}",
  ]

  connection {
    // JW TODO: switch to private ip after VPN is ready
    host        = "${self.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  tags {
    Name          = "builder-api-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "null_resource" "api_provision" {
  triggers {
    ebs_volume = "${aws_volume_attachment.api.id}"
  }

  connection {
    host        = "${aws_instance.api.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:api\" /etc/dd-agent/datadog.conf",
    ]
  }

  provisioner "file" {
    source = "${path.module}/files/nginx.yaml"
    destination = "/tmp/nginx.yaml"
  }

  provisioner "file" {
    source = "${path.module}/files/nginx.logrotate"
    destination = "/tmp/nginx.logrotate"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo cp /tmp/nginx.yaml /etc/dd-agent/conf.d/nginx.yaml",
      "sudo cp /tmp/nginx.logrotate /etc/logrotate.d/nginx",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-api core/builder-api-proxy",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-api --group ${var.env} --bind router:builder-router.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
      "sudo hab svc load core/builder-api-proxy --group ${var.env} --bind http:builder-api.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
    ]
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
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:admin\" /etc/dd-agent/datadog.conf",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh  core/builder-admin core/builder-admin-proxy",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-admin --group ${var.env} --bind router:builder-router.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
      "sudo hab svc load core/builder-admin-proxy --group ${var.env} --bind http:builder-admin.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
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
    "${var.events_sg}",
    "${aws_security_group.datastore.id}",
  ]

  connection {
    // JW TODO: switch to private ip after VPN is ready
    host        = "${self.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  tags {
    Name          = "builder-datastore-${count.index}"
    X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
    X-Environment = "${var.env}"
    X-Application = "builder"
    X-ManagedBy   = "Terraform"
  }
}

resource "null_resource" "datastore_provision" {
  triggers {
    ebs_volume = "${aws_volume_attachment.database.id}"
  }

  connection {
    host        = "${aws_instance.datastore.public_ip}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:datastore\" /etc/dd-agent/datadog.conf",
    ]
  }

  provisioner "file" {
    source = "${path.module}/files/postgres.yaml"
    destination = "/tmp/postgres.yaml"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo awk 'BEGIN{getline l < \"/hab/svc/builder-datastore/config/pwfile\"}/REPLACETHIS/{gsub(\"REPLACETHIS\",l)}1' /tmp/postgres.yaml > /tmp/postgres.yaml.rendered",
      "sudo cp /tmp/postgres.yaml.rendered /etc/dd-agent/conf.d/postgres.yaml",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sumo_sources_worker.rendered}"
    destination = "/home/ubuntu/sumo_sources_worker.json"
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-datastore",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-datastore --group ${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}"
    ]
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
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sch_log_parser.rendered}"
    destination = "/tmp/sch_log_parser.py"
  }

  provisioner "file" {
    source = "${path.module}/files/builder.logrotate"
    destination = "/tmp/builder.logrotate"
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a dogstreams: /tmp/builder-scheduler.log:/etc/dd-agent/sch_log_parser.py:my_log_parser\" /etc/dd-agent/datadog.conf",
      "sudo sed -i \"$ a tags: env:${var.env}, role:jobsrv\" /etc/dd-agent/datadog.conf",
      "sudo cp /tmp/sch_log_parser.py /etc/dd-agent/sch_log_parser.py",
      "sudo cp /tmp/builder.logrotate /etc/logrotate.d/builder",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-jobsrv",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-jobsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}"
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
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:originsrv\" /etc/dd-agent/datadog.conf",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-originsrv",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-originsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}"
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
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:router\" /etc/dd-agent/datadog.conf",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-router",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-router --group ${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}"
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
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:sessionsrv\" /etc/dd-agent/datadog.conf",
      "sudo /etc/init.d/datadog-agent start"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-sessionsrv",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-sessionsrv --group ${var.env} --bind router:builder-router.${var.env} --bind datastore:builder-datastore.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}"
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
  instance_type = "${var.instance_size_worker}"
  key_name      = "${var.aws_key_pair}"
  // JW TODO: switch to private subnet after VPN is ready
  subnet_id     = "${var.public_subnet_id}"
  count         = "${var.jobsrv_worker_count}"

  vpc_security_group_ids = [
    "${var.aws_admin_sg}",
    "${var.hab_sup_sg}",
    "${var.events_sg}",
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

  provisioner "file" {
    source = "${path.module}/scripts/install_base_packages.sh"
    destination = "/tmp/install_base_packages.sh"
  }

  provisioner "remote-exec" {
    scripts = [
      "${path.module}/scripts/init_filesystem.sh",
      "${path.module}/scripts/foundation.sh",
      "${path.module}/scripts/worker_bootstrap.sh",
    ]
  }

  provisioner "file" {
    source = "${path.module}/files/builder.logrotate"
    destination = "/tmp/builder.logrotate"
  }

  provisioner "remote-exec" {
    inline = [
      "DD_INSTALL_ONLY=true DD_API_KEY=${var.datadog_api_key} /bin/bash -c \"$(curl -L https://raw.githubusercontent.com/DataDog/dd-agent/master/packaging/datadog-agent/source/install_agent.sh)\"",
      "sudo sed -i \"$ a tags: env:${var.env}, role:worker\" /etc/dd-agent/datadog.conf",
      "sudo cp /tmp/builder.logrotate /etc/logrotate.d/builder",
      "sudo /etc/init.d/datadog-agent stop"
    ]
  }

  provisioner "file" {
    content     = "${data.template_file.sup_service.rendered}"
    destination = "/home/ubuntu/hab-sup.service"
  }

  provisioner "file" {
    content     = "${data.template_file.worker_user_toml.rendered}"
    destination = "/tmp/worker.user.toml"
  }

  provisioner "remote-exec" {
    inline = [
      "sudo mkdir -p /hab/svc/builder-worker",
      "sudo mv /tmp/worker.user.toml /hab/svc/builder-worker/user.toml",
    ]
  }

  provisioner "remote-exec" {
    inline = [
      "chmod +x /tmp/install_base_packages.sh",
      "sudo /tmp/install_base_packages.sh core/builder-worker",

      "sudo mv /home/ubuntu/hab-sup.service /etc/systemd/system/hab-sup.service",
      "sudo systemctl daemon-reload",
      "sudo systemctl start hab-sup",
      "sudo systemctl enable hab-sup",
      "sudo hab svc load core/builder-worker --group ${var.env} --bind jobsrv:builder-jobsrv.${var.env} --bind depot:builder-api-proxy.${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
      "sudo hab svc load core/sumologic --group ${var.env} --strategy at-once --url ${var.bldr_url} --channel ${var.release_channel}",
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
// Additional Networking

resource "aws_network_interface" "worker_studio" {
  subnet_id       = "${var.worker_studio_subnet_id}"
  security_groups = ["${aws_security_group.worker_studio.id}"]
  count           = "${aws_instance.worker.count}"

  attachment {
    instance     = "${aws_instance.worker.*.id[count.index]}"
    device_index = 1
  }
}

resource "null_resource" "worker_studio_network" {
  count = "${aws_network_interface.worker_studio.count}"

  triggers {
    network_interface_id = "${element(aws_network_interface.worker_studio.*.id, count.index)}"
  }

  connection {
    host        = "${element(aws_instance.worker.*.public_ip, count.index)}"
    user        = "ubuntu"
    private_key = "${file("${var.connection_private_key}")}"
    agent       = "${var.connection_agent}"
  }

  provisioner "file" {
    source = "${path.module}/files/51-studio-init.cfg"
    destination = "/tmp/51-studio-init.cfg"
  }

  provisioner "remote-exec" {
    // This sleep appears to be required. This provisioner runs and the network interface still
    // hasn't been attached.
    inline = [
      "sudo mv /tmp/51-studio-init.cfg /etc/network/interfaces.d/51-studio-init.cfg",
      "sleep 60",
      "sudo systemctl restart networking.service",
    ]
  }
}

////////////////////////////////
// Template Files

data "template_file" "sup_service" {
  template = "${file("${path.module}/templates/hab-sup.service")}"

  vars {
    flags     = "--auto-update --peer ${join(" ", var.peers)} --channel ${var.release_channel} --events hab-eventsrv.default --listen-gossip 0.0.0.0:${var.gossip_listen_port} --listen-http 0.0.0.0:${var.http_listen_port}"
    log_level = "${var.log_level}"
  }
}

data "template_file" "sch_log_parser" {
  template = "${file("${path.module}/templates/sch_log_parser.py")}"

  vars {
    bldr_url = "${var.bldr_url}"
  }
}

data "template_file" "sumo_sources_worker" {
  template = "${file("${path.module}/templates/sumo_sources_local.json")}"

  vars {
    name = "${var.env}"
    category = "${var.env}/worker"
    path = "/tmp/builder-worker.log"
  }
}

data "template_file" "worker_user_toml" {
  template = "${file("${path.module}/templates/worker.user.toml")}"

  vars {
    gateway   = "${var.worker_studio_gateway_ip}"
    interface = "ens4"
  }
}
