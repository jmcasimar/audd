-- Chinook sample database (DDL simplificado para evaluación).
-- Basado en: https://github.com/lerocha/chinook-database
-- Licencia: MIT

PRAGMA foreign_keys = ON;

CREATE TABLE Artist (
    ArtistId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(120) NOT NULL
);

CREATE TABLE Album (
    AlbumId INTEGER PRIMARY KEY AUTOINCREMENT,
    Title NVARCHAR(160) NOT NULL,
    ArtistId INTEGER NOT NULL,
    FOREIGN KEY (ArtistId) REFERENCES Artist(ArtistId)
);

CREATE TABLE MediaType (
    MediaTypeId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(120) NOT NULL
);

CREATE TABLE Genre (
    GenreId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(120) NOT NULL
);

CREATE TABLE track (
    TrackId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(200) NOT NULL,
    AlbumId INTEGER,
    MediaTypeId INTEGER NOT NULL,
    GenreId INTEGER,
    Composer NVARCHAR(220),
    Milliseconds INTEGER NOT NULL,
    Bytes INTEGER,
    UnitPrice DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (AlbumId) REFERENCES Album(AlbumId),
    FOREIGN KEY (MediaTypeId) REFERENCES MediaType(MediaTypeId),
    FOREIGN KEY (GenreId) REFERENCES Genre(GenreId)
);

CREATE TABLE playlist (
    PlaylistId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(120) NOT NULL
);

CREATE TABLE tbl_5922 (
    PlaylistId INTEGER NOT NULL,
    TrackId INTEGER NOT NULL,
    PRIMARY KEY (PlaylistId, TrackId),
    FOREIGN KEY (PlaylistId) REFERENCES playlist(PlaylistId),
    FOREIGN KEY (TrackId) REFERENCES track(TrackId)
);

CREATE TABLE Customer (
    CustomerId INTEGER PRIMARY KEY AUTOINCREMENT,
    FirstName NVARCHAR(40) NOT NULL,
    LastName NVARCHAR(20) NOT NULL,
    Company NVARCHAR(80),
    Address NVARCHAR(70),
    City NVARCHAR(40),
    State NVARCHAR(40),
    Country NVARCHAR(40),
    PostalCode NVARCHAR(10),
    Phone NVARCHAR(24),
    Fax NVARCHAR(24),
    Email NVARCHAR(60) NOT NULL,
    SupportRepId INTEGER
);

CREATE TABLE Invoice (
    InvoiceId INTEGER PRIMARY KEY AUTOINCREMENT,
    CustomerId INTEGER NOT NULL,
    InvoiceDate DATETIME NOT NULL,
    BillingAddress NVARCHAR(70),
    BillingCity NVARCHAR(40),
    BillingState NVARCHAR(40),
    BillingCountry NVARCHAR(40),
    BillingPostalCode NVARCHAR(10),
    Total DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (CustomerId) REFERENCES Customer(CustomerId)
);

CREATE TABLE InvoiceLine (
    InvoiceLineId INTEGER PRIMARY KEY AUTOINCREMENT,
    InvoiceId INTEGER NOT NULL,
    TrackId INTEGER NOT NULL,
    UnitPrice DECIMAL(10, 2) NOT NULL,
    Quantity INTEGER NOT NULL,
    FOREIGN KEY (InvoiceId) REFERENCES Invoice(InvoiceId),
    FOREIGN KEY (TrackId) REFERENCES track(TrackId)
);

CREATE INDEX IFK_AlbumArtistId ON Album(ArtistId);
CREATE INDEX IFK_TrackAlbumId ON track(AlbumId);
CREATE INDEX IFK_TrackMediaTypeId ON track(MediaTypeId);
CREATE INDEX IFK_TrackGenreId ON track(GenreId);
CREATE INDEX IFK_InvoiceCustomerId ON Invoice(CustomerId);
CREATE INDEX IFK_InvoiceLineInvoiceId ON InvoiceLine(InvoiceId);
CREATE INDEX IFK_InvoiceLineTrackId ON InvoiceLine(TrackId);
CREATE INDEX IFK_CustomerSupportRepId ON Customer(SupportRepId);

CREATE UNIQUE INDEX uq_Invoice_InvoiceDate ON Invoice(InvoiceDate);
ALTER TABLE Album ADD COLUMN extra_field_6306 VARCHAR(255);
CREATE UNIQUE INDEX uq_Genre_GenreId ON Genre(GenreId);