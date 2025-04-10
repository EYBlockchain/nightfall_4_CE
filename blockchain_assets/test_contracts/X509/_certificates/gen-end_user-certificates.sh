#!/bin/bash
script_dir=$(dirname "$(realpath "$0")")
set -e

usage()
{
  echo "Usage: ./gen-end_user-certificates.sh [user_suffix]"
}

if [ -z "$1" ]; then
   usage
   exit 1
fi

mkdir -p user

user_name=user-$1

echo "Generating self-signed certificate for user '$user_name'"

# Generate the user's private key in DER format
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -out user/$user_name.priv_key -outform DER
echo "We have successfully generated the private key for user '$user_name'"

# Convert the private key to PEM format for use in CSR generation
openssl rsa -inform DER -in user/$user_name.priv_key -out user/$user_name.pem

# Generate the certification request
openssl req -new -subj "/C=IN/ST=Mumbai/O=User-$1/OU=Nightfall Team/CN=$user_name/emailAddress=$user_name@user.com" -key user/$user_name.pem -out user/$user_name.csr
echo "We have successfully generated the certification request for user '$user_name'"

# Convert the intermediate CA private key to PEM format if it is in DER format
if [ ! -f intermediate_ca_private_key.pem ]; then
  openssl rsa -inform DER -in intermediate_ca.priv_key -out intermediate_ca_private_key.pem
fi

# Create the serial number file if it does not exist
if [ ! -f intermediate_ca.srl ]; then
  echo 01 > intermediate_ca.srl
fi

# Convert intermediate_ca.der to PEM Format
openssl x509 -inform DER -in intermediate_ca.der -out intermediate_ca_priv_key.pem

# Dynamically create the extension configuration file
cat > "$script_dir/conf/ca_extfile.conf" <<EOL
[ extensions ]
basicConstraints = CA:FALSE
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = emailProtection, timeStamping
certificatePolicies = @policies

[ policies ]
policyIdentifier = 1.5.6.7
EOL

# Generate a valid certificate in DER format
openssl x509 -req \
   -CA intermediate_ca_priv_key.pem \
  -CAkey intermediate_ca_private_key.pem \
  -CAserial intermediate_ca.srl \
  -days 1825 \
  -extfile "$script_dir/conf/ca_extfile.conf" \
  -extensions extensions \
  -outform DER \
  -in user/$user_name.csr \
  -out user/$user_name.der

# Clean up CSR and PEM file
rm user/$user_name.csr
rm user/$user_name.pem

echo "We have successfully generated the certificate for user '$user_name' in DER format"

rm intermediate_ca_priv_key.pem
rm intermediate_ca_private_key.pem
rm intermediate_ca.srl