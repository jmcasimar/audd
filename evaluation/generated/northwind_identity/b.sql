-- Northwind sample database (DDL simplificado para evaluación).
-- Basado en: https://github.com/jpwhite3/northwind-SQLite3
-- Licencia: MIT

PRAGMA foreign_keys = ON;

CREATE TABLE Categories (
    CategoryID INTEGER PRIMARY KEY AUTOINCREMENT,
    CategoryName VARCHAR(15) NOT NULL,
    Description TEXT
);

CREATE TABLE Suppliers (
    SupplierID INTEGER PRIMARY KEY AUTOINCREMENT,
    CompanyName VARCHAR(40) NOT NULL,
    ContactName VARCHAR(30),
    ContactTitle VARCHAR(30),
    Address VARCHAR(60),
    City VARCHAR(15),
    Region VARCHAR(15),
    PostalCode VARCHAR(10),
    Country VARCHAR(15),
    Phone VARCHAR(24),
    Fax VARCHAR(24),
    HomePage TEXT
);

CREATE TABLE Products (
    ProductID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    ProductName VARCHAR(40) NOT NULL,
    SupplierID INTEGER,
    CategoryID INTEGER,
    QuantityPerUnit VARCHAR(20),
    UnitPrice DECIMAL(10, 4) DEFAULT 0,
    UnitsInStock SMALLINT DEFAULT 0,
    UnitsOnOrder SMALLINT DEFAULT 0,
    ReorderLevel SMALLINT DEFAULT 0,
    Discontinued BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (CategoryID) REFERENCES Categories(CategoryID),
    FOREIGN KEY (SupplierID) REFERENCES Suppliers(SupplierID)
);

CREATE TABLE Customers (
    CustomerID VARCHAR(5) PRIMARY KEY,
    CompanyName VARCHAR(40) NOT NULL,
    ContactName VARCHAR(30),
    ContactTitle VARCHAR(30),
    Address VARCHAR(60),
    City VARCHAR(15),
    Region VARCHAR(15),
    PostalCode VARCHAR(10),
    Country VARCHAR(15),
    Phone VARCHAR(24),
    Fax VARCHAR(24)
);

CREATE TABLE Employees (
    EmployeeID INTEGER PRIMARY KEY AUTOINCREMENT,
    LastName VARCHAR(20) NOT NULL,
    FirstName VARCHAR(10) NOT NULL,
    Title VARCHAR(30),
    TitleOfCourtesy VARCHAR(25),
    BirthDate DATETIME,
    HireDate DATETIME,
    Address VARCHAR(60),
    City VARCHAR(15),
    Region VARCHAR(15),
    PostalCode VARCHAR(10),
    Country VARCHAR(15),
    HomePhone VARCHAR(24),
    Extension VARCHAR(4),
    Notes TEXT,
    ReportsTo INTEGER,
    FOREIGN KEY (ReportsTo) REFERENCES Employees(EmployeeID)
);

CREATE TABLE Shippers (
    ShipperID INTEGER PRIMARY KEY AUTOINCREMENT,
    CompanyName VARCHAR(40) NOT NULL,
    Phone VARCHAR(24)
);

CREATE TABLE Orders (
    OrderID INTEGER PRIMARY KEY AUTOINCREMENT,
    CustomerID VARCHAR(5),
    EmployeeID INTEGER,
    OrderDate DATETIME,
    RequiredDate DATETIME,
    ShippedDate DATETIME,
    ShipVia INTEGER,
    Freight DECIMAL(10, 4) DEFAULT 0,
    ShipName VARCHAR(40),
    ShipAddress VARCHAR(60),
    ShipCity VARCHAR(15),
    ShipRegion VARCHAR(15),
    ShipPostalCode VARCHAR(10),
    ShipCountry VARCHAR(15),
    FOREIGN KEY (CustomerID) REFERENCES Customers(CustomerID),
    FOREIGN KEY (EmployeeID) REFERENCES Employees(EmployeeID),
    FOREIGN KEY (ShipVia) REFERENCES Shippers(ShipperID)
);

CREATE TABLE OrderDetails (
    OrderID INTEGER NOT NULL,
    ProductID INTEGER NOT NULL,
    UnitPrice DECIMAL(10, 4) NOT NULL DEFAULT 0,
    Quantity SMALLINT NOT NULL DEFAULT 1,
    Discount REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (OrderID, ProductID),
    FOREIGN KEY (OrderID) REFERENCES Orders(OrderID),
    FOREIGN KEY (ProductID) REFERENCES Products(ProductID)
);

CREATE INDEX IFK_OrderDetails_OrderID ON OrderDetails(OrderID);
CREATE INDEX IFK_OrderDetails_ProductID ON OrderDetails(ProductID);
CREATE INDEX IFK_Orders_CustomerID ON Orders(CustomerID);
CREATE INDEX IFK_Orders_EmployeeID ON Orders(EmployeeID);
CREATE INDEX IFK_Orders_ShipVia ON Orders(ShipVia);
CREATE INDEX IFK_Products_CategoryID ON Products(CategoryID);
CREATE INDEX IFK_Products_SupplierID ON Products(SupplierID);

ALTER TABLE OrderDetails ADD COLUMN extra_field_8737 VARCHAR(255);
ALTER TABLE Customers DROP COLUMN ContactName;
ALTER TABLE Categories ADD COLUMN extra_field_7386 VARCHAR(255);