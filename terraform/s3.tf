////////////////////////////////
// Job Log Storage in S3
//
// Currently, we need to get a key and secret for this user and put
// those into the jobsrv config. In the future, we may be able to just
// use instance roles.

resource "aws_iam_user" "jobs" {
  name = "jobs-${var.env}"
  force_destroy = false # be explicit here, because we will have access keys
}

// Job log storage; the server retrieves logs on behalf of requestors,
// so this can be pretty locked down.
resource "aws_s3_bucket" "jobs" {
  bucket = "habitat-jobs-${var.env}"
  acl    = "private"
  region = "${var.aws_region}"

  lifecycle {
    prevent_destroy = true
  }

  tags {
    Name          = "habitat-jobs-${var.env}"
    X-Environment = "${var.env}"
    X-ManagedBy   = "Terraform"
  }
}

data "aws_iam_policy_document" "job_user_can_get_and_put_logs" {
  statement = {
    effect = "Allow"
    actions = ["s3:GetObject", "s3:PutObject"]
    resources = ["${aws_s3_bucket.jobs.arn}/*"]
    principals = {
      type = "AWS"
      identifiers = ["${aws_iam_user.jobs.arn}"]
    }
  }
}

resource "aws_s3_bucket_policy" "jobs" {
  bucket = "${aws_s3_bucket.jobs.id}"
  policy = "${data.aws_iam_policy_document.job_user_can_get_and_put_logs.json}"
}
