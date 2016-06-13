resource "aws_iam_user" "www" {
  name = "${var.www_user}"
}

resource "aws_iam_user_policy" "www" {
  name = "${var.www_user}"
  user = "${aws_iam_user.www.name}"

  policy = <<EOF
{
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:DeleteObject",
        "s3:GetObject",
        "s3:ListBucket",
        "s3:PutObject"
      ],
      "Resource": "arn:aws:s3:::${aws_s3_bucket.www.bucket}/*"
    },
    {
      "Action": "s3:ListAllMyBuckets",
      "Effect": "Allow",
      "Resource": "arn:aws:s3:::*"
    }
  ]
}
EOF
}

resource "aws_s3_bucket" "www" {
  bucket = "${var.www_bucket_name}"
  acl    = "public-read"

  website {
    index_document = "index.html"
    error_document = "404/index.html"
  }

  policy = <<EOF
{
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": [
          "*"
        ]
      },
      "Resource": "arn:aws:s3:::${var.www_bucket_name}/*",
      "Action": "s3:GetObject"
    },
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": "arn:aws:iam::${var.aws_account_id}:user/${aws_iam_user.www.name}"
      },
      "Resource": "arn:aws:s3:::${var.www_bucket_name}",
      "Action": "s3:*"
    }
  ]
}
EOF
}
