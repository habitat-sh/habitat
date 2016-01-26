#!/bin/bash
/usr/bin/env ssh -o "StrictHostKeyChecking=no" -i ~/.ssh/id_rsa_bldr_github $1 $2
