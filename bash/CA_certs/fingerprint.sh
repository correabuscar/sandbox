#!/bin/bash

openssl x509 -noout -in .crossbar/client.crt -fingerprint -sha256
openssl x509 -in .crossbar/client.crt -outform der | openssl sha256
