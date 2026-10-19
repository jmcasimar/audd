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

CREATE TABLE tbl_3476 (
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

CREATE TABLE Playlist (
    PlaylistId INTEGER PRIMARY KEY AUTOINCREMENT,
    Name NVARCHAR(120) NOT NULL
);

CREATE TABLE PlaylistTrack (
    PlaylistId INTEGER NOT NULL,
    TrackId INTEGER NOT NULL,
    PRIMARY KEY (PlaylistId, TrackId),
    FOREIGN KEY (PlaylistId) REFERENCES Playlist(PlaylistId),
    FOREIGN KEY (TrackId) REFERENCES tbl_3476(TrackId)
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
    FOREIGN KEY (TrackId) REFERENCES tbl_3476(TrackId)
);

CREATE INDEX IFK_AlbumArtistId ON Album(ArtistId);
CREATE INDEX IFK_TrackAlbumId ON tbl_3476(AlbumId);
CREATE INDEX IFK_TrackMediaTypeId ON tbl_3476(MediaTypeId);
CREATE INDEX IFK_TrackGenreId ON tbl_3476(GenreId);
CREATE INDEX IFK_InvoiceCustomerId ON Invoice(CustomerId);
CREATE INDEX IFK_InvoiceLineInvoiceId ON InvoiceLine(InvoiceId);
CREATE INDEX IFK_InvoiceLineTrackId ON InvoiceLine(TrackId);
CREATE INDEX IFK_CustomerSupportRepId ON Customer(SupportRepId);

ALTER TABLE Artist ADD COLUMN extra_field_4361 VARCHAR(255);
CREATE UNIQUE INDEX uq_Artist_ArtistId ON Artist(ArtistId);
ALTER TABLE Artist DROP COLUMN Name;