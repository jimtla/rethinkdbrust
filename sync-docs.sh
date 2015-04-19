#!/bin/bash
cd target/doc
rsync -avh . root@sourust.rs:/var/www/rust_docs
