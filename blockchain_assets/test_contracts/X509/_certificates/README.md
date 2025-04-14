# X.509 Certificate Generation and Validation
This document outlines the process of generating X.509 certificates for testing purposes, the scripts utilize OpenSSL to generate a certificate chain, including a root CA, an intermediate CA, and end-user certificates, which are subsequently used for testing and validation in X509 smart contract. Note that root certificates and intemediate certificates should be generated before having valid and invalid end-user certificates. If you regenerate root certificates and intemediate certificates, you should also regenerate end-user certificates, otherwise the validation chain is not correct. 

In the X509 test contract in `nightfall_4/blockchain_assets/test_contracts/X509/X509_t.sol`, we use pre-generated certificates, we suggeset to rename the files when running scripts, otherwise `X509_t.sol` will fail.

## Overview

### Valid Root and Intemediate Certificates
```sh
./gen-root_ca-certificate.sh
```
This script generates the root certificate authority (CA) and an intermediate CA certificate. The intermediate CA is signed by the root CA. After running this file, we can establish the trust chain by creating a root CA and an intermediate CA.

This will generate the followings:
- `conf/ca_extfile.conf`: It defines specific attributes like key usage, extended key usage, and certificate policies for both CA and end-user certificates. 
- `root_ca.crt`: Root CA certificate (PEM format), it's used to extract the authority key identifier and public key modulus.
- `root_ca_priv_key.pem`: Root CA private key (PEM format), it's used to extract modulus.
- `intermediate_ca.der`: Intermediate CA certificate (DER format), it's used for validation during the certificate chain verification process.
- `intermediate_ca_priv_key.pem`: Intermediate_ca private key in PEM format for use in certificate signing request.
- `intermediate_ca.priv_key`: intermediate CA private key.

Remember that after regenerating the root & intermediate CA
certs, the users certificate should also be regenerated!

### For seeing the certificate details
```sh
openssl x509 -noout -text -in root_ca.crt
```
or 
```sh
openssl x509 -noout -text -in intermediate_ca.der
```
### Valid End-User Certificates

To generate a valid end-user certificate, use the `gen-end_user-certificates.sh` script with the user's suffix. For example, `./gen-end_user-certificates.sh 1` will generate certificate files for `user-1`. Valid certificates are stored in the `_certificates/user` folder.
```sh
./gen-end_user-certificates.sh 1
```
This will output the following files:
- `user-1.der`: user's CA in DER fomat
- `user-1.priv_key`: user's private key associated with the certificate, later we will use this private key to sign the client address in `lib/src/validate_certificate.rs`.

### Verify certificates

#### Verify the Intermediate Certificate Against the Root CA
Run the following command to ensure that the intermediate CA certificate (intermediate_ca.crt) is properly signed by the root CA (root_ca.crt):
```sh
openssl x509 -inform DER -in intermediate_ca.der -out intermediate_ca.pem
openssl verify -CAfile root_ca.crt intermediate_ca.pem
```
Expected Output:
```sh
intermediate_ca.pem: OK
```
If this fails, it indicates that the intermediate_ca.crt was not signed by root_ca.crt, or there is an issue with the chain.

#### Verify the End-User Certificate Against the Intermediate and Root CAs
##### Prepare the Chain
To verify the end-user certificate, you need to combine the root and intermediate certificates into a single file (chain.pem):
```sh
cat intermediate_ca.pem root_ca.crt > chain.pem
```
Run the following command to verify user-1.der against the chain:
```sh
openssl verify -CAfile chain.pem user/user-1.der
```
Expected Output:
```sh
user/user-1.der: OK
```
If this fails, it may indicate:
- The user-1.der certificate is not correctly signed by the intermediate_ca.crt.
- The chain is incomplete or improperly formed.



openssl verify -CAfile root_ca.pem intermediate_ca.pem
cat intermediate_ca.pem root_ca.pem > chain.pem
openssl verify -CAfile chain.pem user/user-1.der
