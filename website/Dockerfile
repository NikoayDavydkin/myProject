FROM node:lts
WORKDIR /usr/src/app

COPY ./ ./
COPY ./env.local.prod ./.env.local

# Build the app
RUN npm install
RUN npm run build

# Run the app
CMD ["npm", "start"]
