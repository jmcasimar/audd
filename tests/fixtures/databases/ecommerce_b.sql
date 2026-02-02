-- E-Commerce Test Database Schema (Version B - CamelCase)
-- This database represents the same e-commerce system but with different naming conventions

-- Users table (note: Pascal case table name)
CREATE TABLE Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    userName TEXT NOT NULL UNIQUE,
    emailAddress TEXT NOT NULL,
    createdAt TEXT NOT NULL,
    isActive INTEGER DEFAULT 1
);

-- Products table
CREATE TABLE Products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    productName TEXT NOT NULL,
    productDescription TEXT,
    unitPrice REAL NOT NULL,  -- Note: Using REAL instead of DECIMAL
    categoryName TEXT,
    stockQty INTEGER DEFAULT 0
);

-- Orders table
CREATE TABLE Orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    userId INTEGER NOT NULL,
    productId INTEGER NOT NULL,
    orderQuantity INTEGER NOT NULL DEFAULT 1,
    totalAmount REAL NOT NULL,  -- Note: Using REAL instead of DECIMAL
    orderDateTime TEXT NOT NULL,
    orderStatus TEXT DEFAULT 'pending',
    FOREIGN KEY (userId) REFERENCES Users(id),
    FOREIGN KEY (productId) REFERENCES Products(id)
);

-- Create indexes
CREATE INDEX idx_Orders_userId ON Orders(userId);
CREATE INDEX idx_Orders_productId ON Orders(productId);
CREATE INDEX idx_Orders_orderDate ON Orders(orderDateTime);

-- Create a similar view with different naming
CREATE VIEW OrderDetails AS
SELECT 
    o.id as orderId,
    u.userName,
    u.emailAddress,
    p.productName,
    o.orderQuantity,
    o.totalAmount,
    o.orderDateTime,
    o.orderStatus
FROM Orders o
JOIN Users u ON o.userId = u.id
JOIN Products p ON o.productId = p.id;

-- Insert sample data
INSERT INTO Users (userName, emailAddress, createdAt, isActive) VALUES
('alice', 'alice@example.com', '2024-01-01 10:00:00', 1),
('bob', 'bob@example.com', '2024-01-02 11:00:00', 1),
('charlie', 'charlie@example.com', '2024-01-03 12:00:00', 0);

INSERT INTO Products (productName, productDescription, unitPrice, categoryName, stockQty) VALUES
('Laptop', 'High-performance laptop', 999.99, 'Electronics', 10),
('Mouse', 'Wireless mouse', 29.99, 'Electronics', 50),
('Desk Chair', 'Ergonomic office chair', 199.99, 'Furniture', 15);

INSERT INTO Orders (userId, productId, orderQuantity, totalAmount, orderDateTime, orderStatus) VALUES
(1, 1, 1, 999.99, '2024-01-10 14:00:00', 'completed'),
(2, 2, 2, 59.98, '2024-01-11 15:00:00', 'completed'),
(1, 3, 1, 199.99, '2024-01-12 16:00:00', 'pending');
