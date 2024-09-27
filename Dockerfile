# Start with a Rust image
FROM rustlang/rust:nightly as builder

# Add the wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install trunk

# Set the working directory
WORKDIR /usr/src/app

# Copy your source code
COPY . .

# Build your app
RUN trunk build --release

# Now, let's create a tiny image to serve your app
FROM nginx:alpine

# Copy the built assets from the builder stage
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
