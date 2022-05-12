FROM rust:latest

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs

WORKDIR /usr/src/crabtyper-web

COPY . .

RUN npm install

RUN npm run setup

CMD [ "npm", "run", "prod" ]
