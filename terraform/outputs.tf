output "www_bucket_name" {
  value = "${aws_s3_bucket.www.id}"
}

output "www_user" {
  value = "${aws_iam_user.www.name}"
}
