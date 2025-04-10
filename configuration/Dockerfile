FROM nginx:latest
# install additional ca-certificates e.g. zscaler certificate (can be removed if not needed)
COPY ./configuration/trust/* /usr/local/share/ca-certificates/
RUN chmod 644 /usr/local/share/ca-certificates/* && update-ca-certificates
RUN mkdir -p /var/www/html/configuration/toml
RUN chmod 777 /var/www/html
RUN chmod 777 /var/www/html/configuration
RUN chmod 777 /var/www/html/configuration/toml
COPY ./configuration/toml/addresses.toml /var/www/html/configuration/toml/addresses.toml
COPY ./configuration/nginx.conf /etc/nginx/conf.d/1.conf
COPY ./configuration/bin/* /var/www/html/
CMD ["nginx", "-g", "daemon off;"]