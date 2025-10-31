#!/bin/sh
# Fix permissions at runtime (after volumes are mounted)
chown -R nginx:nginx /var/www/html
chmod -R 755 /var/www/html

# Start nginx
exec nginx -g "daemon off;"
