FROM docker.io/library/fedora:latest

RUN dnf update -y && dnf install -y nginx && dnf clean all

RUN sed -i 's/listen 80;/listen 8080;/' /etc/nginx/nginx.conf

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
