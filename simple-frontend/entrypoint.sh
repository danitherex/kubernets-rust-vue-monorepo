#!/bin/sh

# Replace placeholder in env.js with actual environment variable
echo "window.env = { VUE_APP_API_URL: '${VUE_APP_API_URL:-http://localhost:30100}' };" > /usr/share/nginx/html/env.js

# Start NGINX
exec "$@"
