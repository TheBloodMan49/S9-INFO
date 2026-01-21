FROM nginx:alpine

COPY server/html/ /usr/share/nginx/html/

COPY server/default.conf /etc/nginx/conf.d/default.conf

# Create .htpasswd file with ctfplayer:challenge123 as credentials
RUN apk add --no-cache apache2-utils && htpasswd -b -c /etc/nginx/.htpasswd myst3riOus-Us3r s3cre1P477

EXPOSE 80
