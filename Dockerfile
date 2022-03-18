FROM paritytech/ci-linux:production

WORKDIR /var/www/dot_marketplace_node
COPY . /var/www/dot_marketplace_node
EXPOSE 9944