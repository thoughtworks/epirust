FROM node:10.20.0-alpine
COPY package*.json ./

RUN npm install
RUN mkdir web
COPY . ./web
RUN cd web/server && npm install
RUN cd web/react-spa && npm install && npm rebuild node-sass --force
