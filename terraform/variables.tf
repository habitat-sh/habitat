variable "aws_region" {
  description = "AWS region to launch servers."
  default     = "us-west-2"
}

variable "rhel7_ami" {
  # TODO: Make this a map
  description = "RHEL 7 AMI in us-west-2"
  default     = "ami-99bef1a9"
}

