////////////////////////////////
// Job Log Storage in S3
//
// Currently, we need to get a key and secret for this user and put
// those into the jobsrv config. In the future, we may be able to just
// use instance roles.
resource "aws_iam_user" "jobs" {
  name = "jobs-${var.env}"
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
}

// The job user (and only the job user) can put / get objects in the
// bucket!
resource "aws_s3_bucket_policy" "jobs" {
  bucket = "${aws_s3_bucket.jobs.id}"
  policy = <<EOF
{
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": "${aws_iam_user.jobs.arn}"
      },
      "Resource": "${aws_s3_bucket.jobs.arn}/*",
      "Action": [
        "s3:GetObject",
        "s3:PutObject"
      ]
    }
  ]
}
EOF
}
