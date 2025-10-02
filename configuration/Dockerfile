FROM nginx:latest
# Install additional CA certificates if needed
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates || true

# Remove default nginx config so only ours is used
RUN rm /etc/nginx/conf.d/default.conf

# Create config directory
RUN mkdir -p /var/www/html/configuration/toml \
    && chmod -R 755 /var/www/html

# Copy our nginx config
COPY ./configuration/nginx.conf /etc/nginx/conf.d/default.conf

# Copy static files (if any go into /var/www/html/bin)
COPY ./configuration/bin/* /var/www/html/

CMD ["nginx", "-g", "daemon off;"]
