FROM nginx:latest

COPY ./configuration/trust/ZscalerRootCertificate-2048-SHA256-Feb2025.pem /usr/local/share/ca-certificates/zscaler-root-feb2025.crt
RUN chmod 644 /usr/local/share/ca-certificates/zscaler-root-feb2025.crt && update-ca-certificates || true

# Remove default nginx config so only ours is used
RUN rm /etc/nginx/conf.d/default.conf

# Create config directory
RUN mkdir -p /var/www/html/configuration/toml 

# Copy our nginx config
COPY ./configuration/nginx.conf /etc/nginx/conf.d/default.conf

# Copy static files (if any go into /var/www/html/bin)
COPY ./configuration/bin/keys/proving_key /var/www/html/
COPY ./configuration/bin/keys/deposit_proving_key /var/www/html/

RUN chown -R nginx:nginx /var/www/html \     
    && chmod -R 755 /var/www/html  

CMD ["nginx", "-g", "daemon off;"]


