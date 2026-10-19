-- Grow Venta: esquema anonimizado de punto de tbl_5922 (POS).
-- Solo estructura, sin datos ni identificadores reales.
-- Basado en el caso de estudio Grow Venta vs Grow Travel.

PRAGMA foreign_keys = ON;

CREATE TABLE producto (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_barras TEXT UNIQUE NOT NULL,
    nombre VARCHAR(200) NOT NULL,
    descripcion TEXT,
    precio DECIMAL(12, 2) NOT NULL,
    existencia INTEGER NOT NULL DEFAULT 0,
    categoria_id INTEGER,
    activo BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categoria (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT
);

CREATE TABLE Cliente (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo VARCHAR(20) UNIQUE NOT NULL,
    nombre VARCHAR(200) NOT NULL,
    telefono VARCHAR(20),
    correo VARCHAR(100),
    rfc VARCHAR(13),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tbl_5922 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folio VARCHAR(20) UNIQUE NOT NULL,
    cliente_id INTEGER NOT NULL,
    fecha DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    subtotal DECIMAL(12, 2) NOT NULL,
    impuesto DECIMAL(12, 2) NOT NULL,
    total DECIMAL(12, 2) NOT NULL,
    estatus VARCHAR(20) NOT NULL DEFAULT 'completada',
    FOREIGN KEY (cliente_id) REFERENCES Cliente(id)
);

CREATE TABLE venta_detalle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    producto_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    precio_unitario DECIMAL(12, 2) NOT NULL,
    importe DECIMAL(12, 2) NOT NULL,
    FOREIGN KEY (venta_id) REFERENCES tbl_5922(id),
    FOREIGN KEY (producto_id) REFERENCES producto(id)
);

CREATE TABLE pago (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    forma_pago VARCHAR(20) NOT NULL,
    monto DECIMAL(12, 2) NOT NULL,
    referencia VARCHAR(50),
    fecha DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (venta_id) REFERENCES tbl_5922(id)
);

CREATE INDEX idx_producto_categoria ON producto(categoria_id);
CREATE INDEX idx_venta_cliente ON tbl_5922(cliente_id);
CREATE INDEX idx_venta_fecha ON tbl_5922(fecha);
CREATE INDEX idx_detalle_venta ON venta_detalle(venta_id);
CREATE INDEX idx_detalle_producto ON venta_detalle(producto_id);

CREATE UNIQUE INDEX uq_venta_detalle_importe ON venta_detalle(importe);
CREATE UNIQUE INDEX uq_cliente_created_at ON Cliente(created_at);