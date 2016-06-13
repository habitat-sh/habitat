resource "aws_instance" "builder_api" {
    ami           = "${lookup(var.aws_ami, var.aws_region)}"
    instance_type = "t2.medium"
    key_name      = "${var.aws_key_pair}"
    subnet_id     = "${var.public_subnet_id}"
    count         = "${var.rest_api_count}"

    vpc_security_group_ids = [
        "${var.aws_admin_sg}",
        "${var.hab_sup_sg}",
        "${aws_security_group.builder_api.id}",
        "${aws_security_group.router_gateway.id}",
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
            "sudo ln -s /mnt/hab /hab"
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
            "${template_file.gateway_director.rendered}",
            "BODY",
            "sudo mv /tmp/director-config.toml /hab/etc/director/config.toml",
            "sudo systemctl daemon-reload",
            "sudo systemctl start hab-director",
            "sudo systemctl enable hab-director",
        ]
    }

    tags {
        Name          = "builder-api-${count.index}"
        X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
        X-Environment = "${var.env}"
        X-Application = "builder"
    }
}

resource "template_file" "gateway_director" {
    template = "${file("${path.module}/templates/gateway-director.toml")}"

    vars {
        env     = "${var.env}"
        // peer_ip = "${aws_instance.router.0.private_ip}"
        peer_ip = "${aws_instance.monolith.0.private_ip}"
    }
}

resource "aws_security_group" "builder_api_elb" {
    name        = "builder-api-elb-${var.env}"
    description = "Habitat Builder API Load Balancer"
    vpc_id      = "${var.aws_vpc_id}"

    // JW TODO: remove after old clients are retired
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
        X-Contact     = "The Habitat Maintainers <humans@habitat.sh>"
        X-Environment = "${var.env}"
        X-Application = "builder"
    }
}

resource "aws_security_group" "builder_api" {
    name        = "builder-api-${var.env}"
    description = "For traffic to builder-api network services"
    vpc_id      = "${var.aws_vpc_id}"

    egress {
        from_port   = 0
        to_port     = 0
        protocol    = "-1"
        cidr_blocks = ["0.0.0.0/0"]
    }

    ingress {
        from_port       = 9636
        to_port         = 9636
        protocol        = "tcp"
        security_groups = [
            "${aws_security_group.builder_api_elb.id}"
        ]
    }

    tags {
        X-Environment  = "${var.env}"
        X-Contact      = "The Habitat Maintainers <humans@habitat.sh>"
        X-Application = "builder"
    }
}

resource "aws_elb" "builder_api" {
    name            = "builder-api"
    security_groups = ["${aws_security_group.builder_api_elb.id}"]
    subnets         = ["${var.public_subnet_id}"]
    instances       = ["${aws_instance.monolith.*.id}"]

    listener {
        instance_port      = 9636
        instance_protocol  = "HTTP"
        lb_port            = 443
        lb_protocol        = "HTTPS"
        ssl_certificate_id = "${var.ssl_certificate_arn}"
    }

    // JW TODO: remove after old clients are retired
    listener {
        instance_port      = 9636
        instance_protocol  = "HTTP"
        lb_port            = 80
        lb_protocol        = "HTTP"
    }

    health_check {
        healthy_threshold   = 2
        unhealthy_threshold = 10
        timeout             = 30
        target              = "HTTP:9636/v1/status"
        interval            = 60
    }

    tags {
        X-Environment = "${var.env}"
        X-Application = "builder"
    }
}
