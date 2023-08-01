# TNCars Actix

REST backend for TNCars, written using Actix-Web and sqlx.

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
- [ ] Better error handling throughout (implement AppError)
- [ ] Better implementation of protected routes
- [ ] Proper returning sql queries
- [ ] Convert messaging to websockets

## About the project

This is a personal project of mine, an instagram like site for cars,
built for the local community. This repository contains the REST backend
for the site.

The code quality might not be exemplary, but it is great for didactic purposes
especially for beginners.

Anyone looking for better implementations, look into the real-world repo
[here](https://github.com/gothinkster/realworld).
