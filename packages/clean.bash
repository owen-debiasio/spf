#!/bin/bash

echo "Press enter to clean ./Packages/"
read

rm ./packages/spf-* || echo "No need to clean"