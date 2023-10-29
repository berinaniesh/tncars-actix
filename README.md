# TNCars Actix

REST backend for TNCars, written using Actix-Web and sqlx.

This project is being rewritten with "idiomatic rust" and "best practices"
(like refresh tokens, error propagation, swagger docs, etc). 

This repo probably won't receive updates anymore. 

## To Do (Short Term)

- [x] Delete the old pic when profile pics are updated
- [x] Add endpoints for password reset
- [x] Add endpoints for uploading post pics
- [ ] Add endpoints for direct messages
- [ ] Add endpoints for uploading images / files in direct messages
- [x] Increase the resize size
- [ ] Move the image resize function to the utils module
- [ ] Functionality to search posts
- [ ] Functionality to search users
- [ ] Build the recommendation system

## To Do (Long Term)

These are long term todos. The codebase can be improved in many areas,
especially in the authentication side.

- [ ] SwaggerUI for API documentation
- [x] Better error handling throughout (implement AppError)
- [ ] Better implementation of protected routes
- [ ] Proper returning sql queries
- [ ] Convert messaging to websockets

## About the project

This is a personal project of mine, an instagram like site for cars,
built for the local community. This repository contains the REST backend
for the site.
