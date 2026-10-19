-- Grow Venta: esquema anonimizado de punto de venta (POS).
-- Solo estructura, sin datos ni identificadores reales.
-- Basado en el caso de estudio Grow Venta vs Grow Travel.

PRAGMA foreign_keys = ON;

CREATE TABLE producto (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo_barras VARCHAR(50) UNIQUE NOT NULL,
    nombre TEXT NOT NULL,
    descripcion TEXT,
    precio DECIMAL(12, 2) NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0,
    categoria_id INTEGER,
    activo BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categoria (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre VARCHAR(100) NOT NULL,
    descripcion TEXT
);

CREATE TABLE cliente (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    codigo VARCHAR(20) UNIQUE NOT NULL,
    nombre VARCHAR(200) NOT NULL,
    telefono VARCHAR(20),
    correo VARCHAR(100),
    rfc VARCHAR(13),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE venta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    folio VARCHAR(20) UNIQUE NOT NULL,
    cliente_id INTEGER NOT NULL,
    date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    subtotal DECIMAL(12, 2) NOT NULL,
    impuesto DECIMAL(12, 2) NOT NULL,
    total DECIMAL(12, 2) NOT NULL,
    estatus VARCHAR(20) NOT NULL DEFAULT 'completada',
    FOREIGN KEY (cliente_id) REFERENCES cliente(id)
);

CREATE TABLE venta_detalle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    producto_id INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    precio_unitario DECIMAL(12, 2) NOT NULL,
    importe DECIMAL(12, 2) NOT NULL,
    FOREIGN KEY (venta_id) REFERENCES venta(id),
    FOREIGN KEY (producto_id) REFERENCES producto(id)
);

CREATE TABLE pago (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    forma_pago VARCHAR(20) NOT NULL,
    monto DECIMAL(12, 2) NOT NULL,
    referencia VARCHAR(50),
    date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (venta_id) REFERENCES venta(id)
);

CREATE INDEX idx_producto_categoria ON producto(categoria_id);
CREATE INDEX idx_venta_cliente ON venta(cliente_id);
CREATE INDEX idx_venta_fecha ON venta(date);
CREATE INDEX idx_detalle_venta ON venta_detalle(venta_id);
CREATE INDEX idx_detalle_producto ON venta_detalle(producto_id);

ALTER TABLE venta ADD COLUMN extra_field_7219 VARCHAR(255);