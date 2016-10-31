variable "aws_account_id" {
  description = "The AWS account ID. Used by bucket policy"
  default     = "799338195663"
}

variable "env" {
  description = "Name of logical server environment for network"
}

variable "dns_zone_id" {
  description = "DNS Zone for all network"
}

variable "aws_ami" {
  description = "Base AMI for all nodes"

  default = {
    us-west-2 = "ami-06b94666"
  }
}

variable "aws_key_pair" {
  description = "AWS Key Pair name for instances"
}

variable "aws_region" {
  description = "AWS Region"
}

variable "aws_vpc_id" {
  description = "VPC resource id to place security groups into"
}

variable "aws_admin_sg" {
  description = "Administration security group for all instances"
}

variable "hab_sup_sg" {
  description = "Identifier for AWS security group for habitat supervisor connectivity"
}

variable "ssl_certificate_arn" {
  description = "Amazon Resource Name (ARN) for the environment's ssl certificate"
}

variable "public_subnet_id" {
  description = "Identifier for public AWS subnet"
}

variable "private_subnet_id" {
  description = "Identifier for private AWS subnet"
}

variable "monolith_count" {
  description = "Number of monolithic servers to start"
}

variable "rest_api_count" {
  description = "Number of Builder REST API servers to start"
}

variable "router_count" {
  description = "Number of RouteSrv to start"
}

variable "service_count" {
  description = "Number of service servers to start"
}

variable "jobsrv_worker_count" {
  description = "Number of JobSrv workers to start"
}

variable "connection_agent" {
  description = "Set to false to disable using ssh-agent to authenticate"
}

variable "connection_private_key" {
  description = "File path to AWS keypair private key"
}
