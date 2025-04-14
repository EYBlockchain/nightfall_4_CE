#!/bin/bash
set -e

echo "Generating self-signed root CA certificate"

# Generate the root CA private key in DER format
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -out root_ca.priv_key -outform DER

# Convert the private key to PEM format for use in certificate generation
openssl rsa -inform DER -in root_ca.priv_key -out root_ca_priv_key.pem -outform PEM

# Create the OpenSSL configuration file for certificate generation
mkdir -p conf
cat > root_ca.cnf <<EOL
[ req ]
distinguished_name = req_distinguished_name
x509_extensions = v3_ca
prompt = no

[ req_distinguished_name ]
C = UK
ST = London
L = London
O = EY
OU = R&D
CN = NF_4
emailAddress = test@test.com

[ v3_ca ]
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
keyUsage = keyCertSign, cRLSign
extendedKeyUsage = emailProtection, timeStamping
certificatePolicies = @policies

[ policies ]
policyIdentifier = 1.5.6.7
EOL

# Generate the root CA certificate using the configuration file
openssl req -new -x509 -days 1825 -key root_ca_priv_key.pem -out root_ca.crt -config root_ca.cnf -extensions v3_ca

###### generates an intermediate certificate
echo "Generating intermediate CA certificate"

# Generate the intermediate CA private key in DER format
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -out intermediate_ca.priv_key -outform DER

# Convert the private key to PEM format for use in certificate signing request
openssl rsa -inform DER -in intermediate_ca.priv_key -out intermediate_ca_priv_key.pem -outform PEM

# Generate the intermediate CA certificate signing request (CSR)
openssl req -new -subj "/C=IN/ST=Mumbai/O=Intermediate CA/OU=Nightfall Team/CN=Intermediate CA/emailAddress=intermediate_ca@ca.com" -key intermediate_ca_priv_key.pem -out intermediate_ca.csr

echo "We have successfully generated the certification request for intermediate CA"

# Create the OpenSSL configuration file for the Intermediate CA certificate
cat > conf/ca_extfile.conf <<EOL
[ extensions ]
basicConstraints = CA:TRUE
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer
keyUsage = digitalSignature, keyEncipherment, keyCertSign, cRLSign
extendedKeyUsage = emailProtection, timeStamping
certificatePolicies = @policies

[ policies ]
policyIdentifier = 1.5.6.7
EOL

# Convert the root CA certificate to PEM format (if it's not already)
openssl x509 -inform PEM -in root_ca.crt -out root_ca.pem -outform PEM

# Convert the root CA private key to PEM format (if it's not already)
openssl rsa -inform DER -in root_ca.priv_key -out root_ca_priv_key.pem -outform PEM

# Sign the Intermediate CA CSR with the root CA
openssl x509 -req \
  -CA root_ca.pem \
  -CAkey root_ca_priv_key.pem \
  -CAcreateserial \
  -days 1825 \
  -extfile conf/ca_extfile.conf \
  -extensions extensions \
  -in intermediate_ca.csr \
  -out intermediate_ca.der \
  -outform DER

echo "Intermediate CA certificate has been generated and signed by the root CA in DER format."

rm root_ca.priv_key
rm root_ca.cnf
rm root_ca.pem
rm root_ca.srl
rm intermediate_ca.csr