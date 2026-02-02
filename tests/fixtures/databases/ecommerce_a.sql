-- E-Commerce Test Database Schema (Version A - snake_case)
-- This database represents an e-commerce system with users, products, and orders

-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_active INTEGER DEFAULT 1
);

-- Products table
CREATE TABLE products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_name TEXT NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    category TEXT,
    stock_quantity INTEGER DEFAULT 0
);

-- Orders table
CREATE TABLE orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    total_price DECIMAL(10,2) NOT NULL,
    order_date TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
);

-- Create indexes for performance
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_product_id ON orders(product_id);
CREATE INDEX idx_orders_order_date ON orders(order_date);

-- Create a view for order summaries
CREATE VIEW order_summary AS
SELECT 
    o.id as order_id,
    u.username,
    u.email,
    p.product_name,
    o.quantity,
    o.total_price,
    o.order_date,
    o.status
FROM orders o
JOIN users u ON o.user_id = u.id
JOIN products p ON o.product_id = p.id;

-- Insert sample data
INSERT INTO users (username, email, created_at, is_active) VALUES
('alice', 'alice@example.com', '2024-01-01 10:00:00', 1),
('bob', 'bob@example.com', '2024-01-02 11:00:00', 1),
('charlie', 'charlie@example.com', '2024-01-03 12:00:00', 0);

INSERT INTO products (product_name, description, price, category, stock_quantity) VALUES
('Laptop', 'High-performance laptop', 999.99, 'Electronics', 10),
('Mouse', 'Wireless mouse', 29.99, 'Electronics', 50),
('Desk Chair', 'Ergonomic office chair', 199.99, 'Furniture', 15);

INSERT INTO orders (user_id, product_id, quantity, total_price, order_date, status) VALUES
(1, 1, 1, 999.99, '2024-01-10 14:00:00', 'completed'),
(2, 2, 2, 59.98, '2024-01-11 15:00:00', 'completed'),
(1, 3, 1, 199.99, '2024-01-12 16:00:00', 'pending');
