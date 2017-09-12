#!/bin/bash

#End to end tests require some normalisation of files to correct for varying formatting by vendors (because the output format is static).

# Normalise hexadecimal case: 
gsed -i 's|\(0x[0-9a-fA-F]*\)|\L\1|g' $1

# Normalise hexadecimal lengths
gsed -i 's|>0x[0]\{1,2\}\([0-9a-f]\{1,2\}\)<|>0x\1<|g' $1

# Swap ST resets to full register width
gsed -i 's|<resetValue>0x00</resetValue>|<resetValue>0x00000000</resetValue>|g' $1

# Swap register sizes from hex to dec
gsed -i 's|<size>0x20</size>|<size>32</size>|g' $1

# Remove displayName props because we don't care
gsed -i 's|\(\s*<displayName>[a-zA-Z0-9]*</displayName>[\r\n]*\)||g' $1

# Remove empty descriptions
gsed -i 's|<description/>||g' $1