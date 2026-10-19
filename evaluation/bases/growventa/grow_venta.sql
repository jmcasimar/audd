CREATE TABLE User (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        username TEXT UNIQUE NOT NULL,
                        password TEXT NOT NULL,
                        email TEXT DEFAULT '',
                        admin_permission INTEGER NOT NULL DEFAULT (0)
                    , pin TEXT DEFAULT NULL);
CREATE TRIGGER update_User AFTER UPDATE ON User
    BEGIN
        UPDATE User SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE IF NOT EXISTS "Setting_discount" (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        min REAL NOT NULL,
                        discount REAL NOT NULL,
                        setting_id INTEGER,
                        FOREIGN KEY (setting_id) REFERENCES Setting (object_id) ON DELETE SET NULL 
                    );
CREATE TABLE Department (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        name TEXT UNIQUE NOT NULL,
                        description TEXT
                    );
CREATE TRIGGER update_Department AFTER UPDATE ON Department
    BEGIN
        UPDATE Department SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Tax (
                        object_id INTEGER PRIMARY KEY,
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        name TEXT UNIQUE NOT NULL,
                        tax_name TEXT NOT NULL DEFAULT "",
                        value REAL NOT NULL,
                        type INTEGER NOT NULL,
                        withholding INTEGER NOT NULL DEFAULT 0,
                        transfer INTEGER NOT NULL DEFAULT 1,
                        apply_default INTEGER NOT NULL DEFAULT 0
                    );
CREATE TRIGGER update_Tax AFTER UPDATE ON Tax
    BEGIN
        UPDATE Tax SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Product (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        code TEXT NOT NULL DEFAULT "",
                        name TEXT UNIQUE NOT NULL,
                        amount REAL NOT NULL,
                        price REAL NOT NULL,
                        cost REAL,
                        user_id INTEGER,
                        department_id INTEGER DEFAULT (1), unit TEXT NOT NULL DEFAULT 'pza', url_image TEXT DEFAULT NULL, is_supply INTEGER NOT NULL DEFAULT 0, unit_purchase TEXT, unit_quantity INTEGER DEFAULT 1, track_inventory INTEGER NOT NULL DEFAULT 1,
                        FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (department_id) REFERENCES Department (object_id)
                    );
CREATE TRIGGER update_Product AFTER UPDATE ON Product
    BEGIN
        UPDATE Product SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER department_product_update BEFORE DELETE ON Department
                        BEGIN
                            UPDATE Product SET department_id=(1) WHERE department_id=OLD.object_id;
                        END;
CREATE TABLE ProductLog (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        user_id INTEGER,
                        product_id INTEGER,
                        price REAL,
                        cost REAL,
                        FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_ProductLog AFTER UPDATE ON ProductLog
    BEGIN
        UPDATE ProductLog SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER product_productLog_update BEFORE UPDATE ON Product
                        WHEN (OLD.price IS NOT NEW.price) OR (OLD.cost IS NOT NEW.cost)
                        BEGIN
                            INSERT INTO ProductLog (user_id, product_id, price, cost) VALUES (NEW.user_id, NEW.object_id, NEW.price, NEW.cost);
                        END;
CREATE TABLE Product_Tax (
                        object_id INTEGER PRIMARY KEY,
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        tax_id INTEGER,
                        product_id INTEGER,
                        FOREIGN KEY (tax_id) REFERENCES Tax (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE CASCADE 
                    );
CREATE TRIGGER update_Product_Tax AFTER UPDATE ON Product_Tax
    BEGIN
        UPDATE Product_Tax SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Product_Product (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        quantity REAL NOT NULL DEFAULT (1),
                        product_id INTEGER,
                        includes_product INTEGER,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (includes_product) REFERENCES Product (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_Product_Product AFTER UPDATE ON Product_Product
    BEGIN
        UPDATE Product_Product SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Customer (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        name TEXT UNIQUE NOT NULL,
                        phone TEXT DEFAULT '',
                        email TEXT DEFAULT '',
                        discount REAL NOT NULL DEFAULT (0),
                        credit REAL NOT NULL DEFAULT (0)
                    , points REAL NOT NULL DEFAULT 0, street TEXT, exterior_number TEXT, interior_number TEXT, city TEXT, state TEXT, country TEXT, postal_code TEXT);
CREATE TRIGGER update_Customer AFTER UPDATE ON Customer
    BEGIN
        UPDATE Customer SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Warehouse (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        name TEXT UNIQUE NOT NULL,
                        description TEXT DEFAULT '',
                        customer_id INTEGER,
                        FOREIGN KEY (customer_id) REFERENCES Customer (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_Warehouse AFTER UPDATE ON Warehouse
    BEGIN
        UPDATE Warehouse SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER customer_warehouse_insert AFTER INSERT ON Customer
                        BEGIN
                            INSERT INTO Warehouse (name, customer_id) VALUES (NEW.name, NEW.object_id);
                        END;
CREATE TRIGGER customer_warehouse_update AFTER UPDATE ON Customer
                        BEGIN
                            UPDATE Warehouse SET name=NEW.name WHERE customer_id=NEW.object_id;
                        END;
CREATE TABLE Inventory (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        diff REAL NOT NULL DEFAULT (0),
                        quantity REAL NOT NULL DEFAULT (0),
                        min_quantity REAL NOT NULL DEFAULT (0),
                        user_id INTEGER,
                        warehouse_id INTEGER,
                        product_id INTEGER,
                        FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (warehouse_id) REFERENCES Warehouse (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_Inventory AFTER UPDATE ON Inventory
    BEGIN
        UPDATE Inventory SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER product_insert AFTER INSERT ON Product
                        BEGIN
                            INSERT INTO Inventory (warehouse_id, product_id) VALUES (1, NEW.object_id);
                        END;
CREATE TRIGGER product_return AFTER UPDATE ON Inventory
                        WHEN NEW.quantity == 0 AND NEW.warehouse_id != 1
                        BEGIN
                            DELETE FROM Inventory WHERE product_id=NEW.product_id AND warehouse_id=NEW.warehouse_id;
                        END;
CREATE TABLE InventoryLog (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        diff REAL NOT NULL,
                        quantity REAL NOT NULL,
                        user_id INTEGER,
                        warehouse_id INTEGER,
                        product_id INTEGER,
                        FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (warehouse_id) REFERENCES Warehouse (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_InventoryLog AFTER UPDATE ON InventoryLog
    BEGIN
        UPDATE InventoryLog SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER inventoryLog_insert BEFORE UPDATE ON Inventory
                        WHEN OLD.quantity IS NOT NEW.quantity
                        BEGIN
                            INSERT INTO InventoryLog (diff, quantity, user_id, warehouse_id, product_id) VALUES (NEW.diff, NEW.quantity, NEW.user_id, NEW.warehouse_id, NEW.product_id);
                        END;
CREATE TABLE Multibox (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        type TEXT DEFAULT 'principal',
                        current_ip TEXT DEFAULT '192.168.56.1',
                        port TEXT DEFAULT '5000',
                        principal_ip TEXT DEFAULT '',
                        connected INTEGER NOT NULL DEFAULT (0),
                        last_connection DATETIME DEFAULT NULL,
                        machine_id TEXT DEFAULT 'a04c3841-53b1-4119-bf7d-91a9c22dfb2f',
                        settings_id INTEGER DEFAULT (1),
                        FOREIGN KEY (settings_id) REFERENCES Setting (object_id) ON DELETE CASCADE
                    );
CREATE TRIGGER update_Multibox AFTER UPDATE ON Multibox
    BEGIN
        UPDATE Multibox SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE IF NOT EXISTS "Order_detail" (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        code TEXT NOT NULL,
                        product TEXT NOT NULL,
                        quantity REAL NOT NULL,
                        price REAL NOT NULL,
                        discount REAL NOT NULL,
                        total REAL NOT NULL,
                        order_id INTEGER,
                        product_id INTEGER,
                        department_id INTEGER, cost REAL NOT NULL DEFAULT 0, unit TEXT NOT NULL DEFAULT 'pza', comments TEXT, is_prepared INTEGER NOT NULL DEFAULT 0,
                        FOREIGN KEY (order_id) REFERENCES Orders (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (department_id) REFERENCES Department (object_id)
                    );
CREATE TABLE IF NOT EXISTS "Order_tax" (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        total REAL NOT NULL,
                        order_id INTEGER,
                        product_id INTEGER,
                        order_array_id INTEGER,
                        FOREIGN KEY (order_id) REFERENCES Orders (object_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES Product (object_id) ON DELETE SET NULL,
                        FOREIGN KEY (order_array_id) REFERENCES "Order_detail" (object_id) ON DELETE CASCADE
                    );
CREATE TABLE CashregisterLog (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        quantity REAL NOT NULL,
                        cash REAL NOT NULL,
                        user_id INTEGER,
                        FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL
                    );
CREATE TRIGGER update_CashregisterLog AFTER UPDATE ON CashregisterLog
    BEGIN
        UPDATE CashregisterLog SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE IF NOT EXISTS "License" (
                        object_id INTEGER PRIMARY KEY, 
                        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                        updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                        demo INTEGER NOT NULL DEFAULT (0),
                        expired_at DATETIME DEFAULT NULL
                    , license_hash TEXT, first_license_hash TEXT, license_id TEXT, machine_id TEXT, issued_at TEXT);
CREATE TABLE IF NOT EXISTS "Setting" (
                object_id INTEGER PRIMARY KEY, 
                created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
                small_logo TEXT DEFAULT './logo192.png',
                big_logo TEXT DEFAULT './GrowVenta.png',
                print_logo TEXT DEFAULT './GrowVenta192.png',
                principal_color TEXT DEFAULT '#FF8500',
                secondary_color TEXT DEFAULT '#FFB564',
                name TEXT DEFAULT 'Grow Venta',
                phone TEXT DEFAULT '55-9110-8340',
                address TEXT DEFAULT 'No asignada',
                bank_account TEXT DEFAULT '',
                bank_name TEXT DEFAULT '',
                bank_clabe TEXT DEFAULT '',
                name_quote TEXT DEFAULT '',
                terms_quote TEXT DEFAULT '',
                transfer_comment TEXT DEFAULT '',
                max_credit REAL DEFAULT (10000),
                discount_customer REAL DEFAULT (10),
                discount_order REAL DEFAULT (15),
                enable_discount INTEGER NOT NULL DEFAULT (0),
                mandatory_reference INTEGER NOT NULL DEFAULT (1),
                usd_price REAL NOT NULL DEFAULT (0),
                enable_bailment INTEGER NOT NULL DEFAULT (0),
                tax_included INTEGER NOT NULL DEFAULT (1)
            , is_restaurant INTEGER NOT NULL DEFAULT 0, currency TEXT NOT NULL DEFAULT 'MXN', second_currency TEXT NOT NULL DEFAULT 'USD', costing_strategy TEXT NOT NULL DEFAULT 'LAST_COST', costing_last_n INTEGER NOT NULL DEFAULT 3, enable_batches INTEGER NOT NULL DEFAULT 0, points_perc REAL NOT NULL DEFAULT 0, quotation_folio_prefix TEXT NOT NULL DEFAULT 'GV-', quotation_folio_min_length INTEGER NOT NULL DEFAULT 4);
CREATE TRIGGER update_Setting AFTER UPDATE ON Setting BEGIN
                    UPDATE Setting SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
                END;
CREATE TRIGGER setting_customer_update_credit BEFORE UPDATE ON Setting
                WHEN OLD.max_credit IS NOT NEW.max_credit
                BEGIN
                    UPDATE Customer SET credit=NEW.max_credit WHERE credit>NEW.max_credit;
                END;
CREATE TRIGGER setting_customer_update_discount BEFORE UPDATE ON Setting
                WHEN OLD.discount_customer IS NOT NEW.discount_customer
                BEGIN
                    UPDATE Customer SET discount=NEW.discount_customer WHERE discount>NEW.discount_customer;
                END;
CREATE TABLE IF NOT EXISTS "Orders" (
    object_id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    total REAL NOT NULL DEFAULT 0,
    toPay REAL NOT NULL DEFAULT 0,
    discount REAL NOT NULL DEFAULT 0,
    paid INTEGER NOT NULL DEFAULT 0,
    layaway INTEGER NOT NULL DEFAULT 0,
    credit INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'MXN',
    box_id INTEGER,
    user_id INTEGER,
    customer_id INTEGER, cost REAL NOT NULL DEFAULT 0, table_id INTEGER, tip REAL NOT NULL DEFAULT 0, type TEXT, comments TEXT, points REAL NOT NULL DEFAULT 0, identifier TEXT DEFAULT NULL, is_cancellable INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (customer_id) REFERENCES Customer (object_id) ON DELETE NO ACTION ON UPDATE NO ACTION,
    FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE CASCADE
);
CREATE TRIGGER update_Orders AFTER UPDATE ON Orders
    BEGIN
        UPDATE Orders SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER customer_order_delete BEFORE DELETE ON Customer
                        BEGIN
                            UPDATE Orders SET customer_id=(1) WHERE customer_id=OLD.object_id;
                        END;
CREATE TABLE IF NOT EXISTS "Order_payment" (
    object_id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    payment_method TEXT NOT NULL,
    reference TEXT NOT NULL,
    total REAL NOT NULL,
    currency TEXT NOT NULL DEFAULT 'MXN',
    currency_amount REAL,
    exchange_rate REAL DEFAULT 1.0,
    box_id INTEGER,
    order_id INTEGER, tip REAL NOT NULL DEFAULT 0,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (order_id) REFERENCES Orders (object_id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE CASCADE
);
CREATE TABLE IF NOT EXISTS "Cashcut" (
    object_id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    quantity REAL NOT NULL,
    cash REAL NOT NULL,
    msg TEXT,
    open INTEGER NOT NULL DEFAULT 0,
    box_id INTEGER,
    user_id INTEGER,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE CASCADE
);
CREATE TRIGGER update_Cashcut AFTER UPDATE ON Cashcut
    BEGIN
        UPDATE Cashcut SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE IF NOT EXISTS "Expense" (
    object_id INTEGER PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT,
    total REAL NOT NULL,
    payment_method TEXT NOT NULL,
    box_id INTEGER,
    user_id INTEGER,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (user_id) REFERENCES User (object_id) ON DELETE SET NULL ON UPDATE NO ACTION,
    FOREIGN KEY (box_id) REFERENCES Multibox (object_id) ON DELETE SET NULL ON UPDATE CASCADE
);
CREATE TRIGGER update_Expense AFTER UPDATE ON Expense
    BEGIN
        UPDATE Expense SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER multibox_delete_orders 
                      BEFORE DELETE ON Multibox
                      BEGIN
                          UPDATE Orders SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
                      END;
CREATE TRIGGER multibox_delete_cashcut
                      BEFORE DELETE ON Multibox
                      BEGIN
                          UPDATE Cashcut SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
                      END;
CREATE TRIGGER multibox_delete_expense
                      BEFORE DELETE ON Multibox
                      BEGIN
                          UPDATE Expense SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
                      END;
CREATE INDEX idx_order_payment_box_id ON Order_payment(box_id);
CREATE TRIGGER update_Order_detail
                AFTER UPDATE ON Order_detail
                BEGIN
                    UPDATE Order_detail SET updated_at=CURRENT_TIMESTAMP WHERE object_id=NEW.object_id;
                END;
CREATE TRIGGER update_Order_payment
                AFTER UPDATE ON Order_payment
                BEGIN
                    UPDATE Order_payment SET updated_at=CURRENT_TIMESTAMP WHERE object_id=NEW.object_id;
                END;
CREATE TRIGGER update_Order_tax
                AFTER UPDATE ON Order_tax
                BEGIN
                    UPDATE Order_tax SET updated_at=CURRENT_TIMESTAMP WHERE object_id=NEW.object_id;
                END;
CREATE TRIGGER update_Setting_discount
                AFTER UPDATE ON Setting_discount
                BEGIN
                    UPDATE Setting_discount SET updated_at=CURRENT_TIMESTAMP WHERE object_id=NEW.object_id;
                END;
CREATE TRIGGER department_orderDetail_delete 
            BEFORE DELETE ON Department
            BEGIN
                UPDATE Order_detail SET department_id=(1) WHERE department_id=OLD.object_id;
            END;
CREATE TRIGGER multibox_delete_payments
            BEFORE DELETE ON Multibox
            BEGIN
                UPDATE Order_payment SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
            END;
CREATE TRIGGER update_License
            AFTER UPDATE ON License
            BEGIN
                UPDATE License SET updated_at=CURRENT_TIMESTAMP WHERE object_id=NEW.object_id;
            END;
CREATE TABLE Tour (
            object_id INTEGER PRIMARY KEY AUTOINCREMENT, 
            user_id INTEGER NOT NULL UNIQUE, 
            dashboard_tour INTEGER NOT NULL DEFAULT 0, customers_tour INTEGER NOT NULL DEFAULT 0, products_tour INTEGER NOT NULL DEFAULT 0, departments_tour INTEGER NOT NULL DEFAULT 0, open_cash_tour INTEGER NOT NULL DEFAULT 0, cash_register_tour INTEGER NOT NULL DEFAULT 0, orders_tour INTEGER NOT NULL DEFAULT 0, inventory_tour INTEGER NOT NULL DEFAULT 0, expenses_tour INTEGER NOT NULL DEFAULT 0, cash_drawer_tour INTEGER NOT NULL DEFAULT 0, close_cash_tour INTEGER NOT NULL DEFAULT 0, reports_tour INTEGER NOT NULL DEFAULT 0, configuration_tour INTEGER NOT NULL DEFAULT 0, daily_sales_report_tour INTEGER NOT NULL DEFAULT 0, weekly_sales_report_tour INTEGER NOT NULL DEFAULT 0, monthly_sales_report_tour INTEGER NOT NULL DEFAULT 0, best_customers_report_tour INTEGER NOT NULL DEFAULT 0, best_products_report_tour INTEGER NOT NULL DEFAULT 0, product_details_report_tour INTEGER NOT NULL DEFAULT 0, product_analysis_report_tour INTEGER NOT NULL DEFAULT 0, payments_report_tour INTEGER NOT NULL DEFAULT 0, credit_statement_report_tour INTEGER NOT NULL DEFAULT 0, credit_general_report_tour INTEGER NOT NULL DEFAULT 0, cashcuts_report_tour INTEGER NOT NULL DEFAULT 0, options_config_tour INTEGER NOT NULL DEFAULT 0, business_config_tour INTEGER NOT NULL DEFAULT 0, users_config_tour INTEGER NOT NULL DEFAULT 0, taxes_config_tour INTEGER NOT NULL DEFAULT 0, database_config_tour INTEGER NOT NULL DEFAULT 0, printer_config_tour INTEGER NOT NULL DEFAULT 0, scale_config_tour INTEGER NOT NULL DEFAULT 0, tpv_config_tour INTEGER NOT NULL DEFAULT 0, multibox_config_tour INTEGER NOT NULL DEFAULT 0, 
            FOREIGN KEY (user_id) REFERENCES User(object_id) ON DELETE CASCADE
        );
CREATE TRIGGER user_tour_insert
                AFTER INSERT ON User
                BEGIN
                    INSERT INTO Tour (user_id) VALUES (NEW.object_id);
                END;
CREATE UNIQUE INDEX idx_user_pin_unique ON User(pin) WHERE pin IS NOT NULL;
CREATE TRIGGER prevent_created_at_update
                BEFORE UPDATE OF created_at ON License
                BEGIN
                    SELECT RAISE (FAIL, 'Imposible actualizar :c');
                END;
CREATE INDEX idx_cashcut_created_at
                ON Cashcut (created_at);
CREATE INDEX idx_expense_created_at
                ON Expense (created_at);
CREATE INDEX idx_expense_updated_at
                ON Expense (updated_at);
CREATE INDEX idx_inventory_warehouse_id
                ON Inventory (warehouse_id);
CREATE INDEX idx_inventory_product_id
                ON Inventory (product_id);
CREATE INDEX idx_warehouse_customer_id
                ON Warehouse (customer_id);
CREATE INDEX idx_orders_created_at
                ON Orders (created_at);
CREATE INDEX idx_orders_updated_at
                ON Orders (updated_at);
CREATE INDEX idx_orders_customer_id
                ON Orders (customer_id);
CREATE INDEX idx_orders_total
                ON Orders (total);
CREATE INDEX idx_customer_name
                ON Customer (name);
CREATE INDEX idx_product_name
                ON Product (name);
CREATE INDEX idx_product_department_id
                ON Product (department_id);
CREATE INDEX idx_user_username
                ON User (username);
CREATE INDEX idx_Product_Tax_product_id_tax_id
                ON Product_Tax (product_id, tax_id);
CREATE INDEX idx_orders_box_id
                ON Orders (box_id);
CREATE INDEX idx_cashcut_box_id
                ON Cashcut (box_id);
CREATE INDEX idx_expense_box_id
                ON Expense (box_id);
CREATE TABLE Product_option_assignment (
        object_id INTEGER PRIMARY KEY,
        created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
        product_id INTEGER NOT NULL,
        product_option_id INTEGER NOT NULL,
        sort_order INTEGER NOT NULL DEFAULT 0,
        UNIQUE(product_id, product_option_id)
    );
CREATE INDEX idx_poa_product ON Product_option_assignment(product_id);
CREATE INDEX idx_poa_option ON Product_option_assignment(product_option_id);
CREATE TABLE Printer (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
            name TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'TICKET',
            is_default INTEGER NOT NULL DEFAULT 0,
            is_enabled INTEGER NOT NULL DEFAULT 1,
            connection_type TEXT NOT NULL,
            usb_mode TEXT NULL,
            usb_vid TEXT DEFAULT '',
            usb_pid TEXT DEFAULT '',
            os_printer_name TEXT NULL,
            net_host TEXT NULL,
            net_port INTEGER NULL DEFAULT 9100,
            ticket_copy INTEGER NOT NULL DEFAULT 1,
            settings TEXT NULL
        );
CREATE TRIGGER update_Printer AFTER UPDATE ON Printer
    BEGIN
        UPDATE Printer SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_printer_default ON Printer(is_default);
CREATE INDEX idx_printer_enabled ON Printer(is_enabled);
CREATE INDEX idx_printer_role ON Printer(role);
CREATE INDEX idx_printer_conn ON Printer(connection_type);
CREATE TABLE IF NOT EXISTS "Setting_local"(
  object_id INT,
  created_at NUM,
  updated_at NUM,
  setting_id INT,
  machine_id TEXT,
  backup_path TEXT,
  scale_vid TEXT,
  scale_pid TEXT,
  cash REAL,
  enable_scale INT,
  user_id INT,
  version TEXT,
  channel TEXT,
  tour_login INT,
  touch_mode INT,
  is_commander INT,
  is_kitchen_display INT,
  auto_updates_enabled INT,
  allow_unstable_channel INT,
  product_grid_view INT,
  is_price_checker INT,
  uuid_baseboard TEXT,
  uuid_ram TEXT,
  uuid_disk TEXT,
  uuid_system TEXT,
  platform TEXT
);
CREATE TRIGGER update_Setting_local AFTER UPDATE ON Setting_local
    BEGIN
        UPDATE Setting_local SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Department_Printer (
            department_id INTEGER NOT NULL,
            printer_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            PRIMARY KEY (department_id, printer_id),
            FOREIGN KEY (department_id) REFERENCES Department(object_id) ON DELETE CASCADE,
            FOREIGN KEY (printer_id) REFERENCES Printer(object_id) ON DELETE CASCADE
        );
CREATE TRIGGER update_Department_Printer AFTER UPDATE ON Department_Printer
    BEGIN
        UPDATE Department_Printer SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_dept_printer_dept ON Department_Printer(department_id);
CREATE INDEX idx_dept_printer_printer ON Department_Printer(printer_id);
CREATE TABLE Payment_methods (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            label TEXT NOT NULL,
            value TEXT NOT NULL UNIQUE,
            is_active INTEGER NOT NULL DEFAULT 1
        );
CREATE TRIGGER update_Payment_methods AFTER UPDATE ON Payment_methods
    BEGIN
        UPDATE Payment_methods SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE UNIQUE INDEX idx_payment_methods_value ON Payment_methods(value);
CREATE INDEX idx_payment_methods_active ON Payment_methods(is_active);
CREATE TABLE Quotation (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            folio TEXT UNIQUE NOT NULL,
            total REAL NOT NULL DEFAULT (0),
            toPay REAL NOT NULL DEFAULT (0),
            discount REAL NOT NULL DEFAULT (0),
            currency TEXT NOT NULL DEFAULT 'MXN',
            is_converted INTEGER NOT NULL DEFAULT 0,
            converted_order_id INTEGER NULL,
            terms_quote TEXT NULL,
            transfer_comment TEXT NULL,
            box_id INTEGER,
            user_id INTEGER,
            customer_id INTEGER,
            FOREIGN KEY (user_id) REFERENCES User(object_id) ON DELETE SET NULL,
            FOREIGN KEY (customer_id) REFERENCES Customer(object_id),
            FOREIGN KEY (box_id) REFERENCES Multibox(object_id) ON DELETE SET NULL,
            FOREIGN KEY (converted_order_id) REFERENCES Orders(object_id) ON DELETE SET NULL
        );
CREATE TRIGGER update_Quotation AFTER UPDATE ON Quotation
    BEGIN
        UPDATE Quotation SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER customer_quotation_delete 
            BEFORE DELETE ON Customer
            BEGIN
                UPDATE Quotation SET customer_id=1 WHERE customer_id=OLD.object_id;
            END;
CREATE TRIGGER multibox_delete_quotation 
            BEFORE DELETE ON Multibox
            BEGIN
                UPDATE Quotation SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
            END;
CREATE UNIQUE INDEX idx_quotation_folio ON Quotation(folio);
CREATE INDEX idx_quotation_customer ON Quotation(customer_id);
CREATE INDEX idx_quotation_box ON Quotation(box_id);
CREATE INDEX idx_quotation_converted ON Quotation(is_converted);
CREATE INDEX idx_quotation_created_at ON Quotation(created_at);
CREATE TABLE Quotation_detail (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            code TEXT NOT NULL,
            product TEXT NOT NULL,
            quantity REAL NOT NULL,
            price REAL NOT NULL,
            discount REAL NOT NULL,
            total REAL NOT NULL,
            quotation_id INTEGER,
            product_id INTEGER,
            department_id INTEGER, unit TEXT NOT NULL DEFAULT 'pza', comments TEXT,
            FOREIGN KEY (quotation_id) REFERENCES Quotation(object_id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES Product(object_id) ON DELETE SET NULL,
            FOREIGN KEY (department_id) REFERENCES Department(object_id)
        );
CREATE TRIGGER update_Quotation_detail AFTER UPDATE ON Quotation_detail
    BEGIN
        UPDATE Quotation_detail SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_quotation_detail_quotation ON Quotation_detail(quotation_id);
CREATE INDEX idx_quotation_detail_product ON Quotation_detail(product_id);
CREATE TABLE Unit (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            name TEXT NOT NULL UNIQUE,
            is_enabled INTEGER NOT NULL DEFAULT 1
        );
CREATE TRIGGER update_Unit AFTER UPDATE ON Unit
    BEGIN
        UPDATE Unit SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TABLE Restaurant_tables (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
            name TEXT NOT NULL,
            area TEXT,
            is_active INTEGER NOT NULL DEFAULT 1 
            , priority INTEGER NOT NULL DEFAULT 0);
CREATE TRIGGER update_Restaurant_tables AFTER UPDATE ON Restaurant_tables
    BEGIN
        UPDATE Restaurant_tables SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_restaurant_tables_active ON Restaurant_tables(is_active);
CREATE INDEX idx_orders_table_id ON Orders(table_id);
CREATE INDEX idx_product_department ON Product(department_id);
CREATE TABLE Recipe_items (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), 
            recipe_id INTEGER NOT NULL,
            food_supply_id INTEGER NOT NULL,
            quantity REAL NOT NULL DEFAULT 0,
            FOREIGN KEY (recipe_id) REFERENCES Product (object_id) ON DELETE CASCADE,
            FOREIGN KEY (food_supply_id) REFERENCES Product (object_id) ON DELETE CASCADE
        );
CREATE TRIGGER update_Recipe_items AFTER UPDATE ON Recipe_items
    BEGIN
        UPDATE Recipe_items SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_recipe_items_recipe ON Recipe_items(recipe_id);
CREATE INDEX idx_recipe_items_supply ON Recipe_items(food_supply_id);
CREATE INDEX idx_product_is_supply ON Product(is_supply);
CREATE TABLE Product_options (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            product_id INTEGER,
            name TEXT NOT NULL,
            selection_mode TEXT NOT NULL DEFAULT 'SINGLE',
            is_required INTEGER NOT NULL DEFAULT 0,
            sort_order INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1
        );
CREATE TRIGGER update_Product_options AFTER UPDATE ON Product_options
    BEGIN
        UPDATE Product_options SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_product_options_product ON Product_options(product_id);
CREATE INDEX idx_product_options_active ON Product_options(is_active);
CREATE TABLE Product_option_detail (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            product_option_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            price_delta REAL NOT NULL DEFAULT 0,
            sku_suffix TEXT,
            sort_order INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (product_option_id) REFERENCES Product_options (object_id) ON DELETE CASCADE
        );
CREATE TRIGGER update_Product_option_detail AFTER UPDATE ON Product_option_detail
    BEGIN
        UPDATE Product_option_detail SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_product_option_detail_option ON Product_option_detail(product_option_id);
CREATE TABLE Order_detail_options (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            order_detail_id INTEGER NOT NULL,
            product_option_id INTEGER NOT NULL,
            product_option_detail_id INTEGER NOT NULL,
            name_snapshot TEXT NOT NULL,
            price_delta_snapshot REAL NOT NULL DEFAULT 0
        );
CREATE TRIGGER update_Order_detail_options AFTER UPDATE ON Order_detail_options
    BEGIN
        UPDATE Order_detail_options SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_order_detail_options_detail ON Order_detail_options(order_detail_id);
CREATE TABLE Suppliers (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            name TEXT NOT NULL,
            phone TEXT,
            email TEXT,
            address TEXT,
            is_active INTEGER NOT NULL DEFAULT 1
        );
CREATE TRIGGER update_Suppliers AFTER UPDATE ON Suppliers
    BEGIN
        UPDATE Suppliers SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_suppliers_active ON Suppliers(is_active);
CREATE INDEX idx_suppliers_name ON Suppliers(name);
CREATE TABLE Purchase_orders (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            supplier_id INTEGER,
            status TEXT NOT NULL DEFAULT 'REQUEST',
            currency TEXT NOT NULL DEFAULT 'MXN',
            currency_exchange REAL NOT NULL DEFAULT 1,
            additionals REAL NOT NULL DEFAULT 0,
            notes TEXT,
            paid_from_cash TEXT DEFAULT NULL,
            cash_movement_id INTEGER,
            FOREIGN KEY (supplier_id) REFERENCES Suppliers(object_id) ON DELETE SET NULL,
            FOREIGN KEY (cash_movement_id) REFERENCES Cash_movements(object_id) ON DELETE SET NULL
        );
CREATE TRIGGER update_Purchase_orders AFTER UPDATE ON Purchase_orders
    BEGIN
        UPDATE Purchase_orders SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_purchase_orders_status ON Purchase_orders(status);
CREATE INDEX idx_purchase_orders_supplier ON Purchase_orders(supplier_id);
CREATE INDEX idx_purchase_orders_created_at ON Purchase_orders(created_at);
CREATE TABLE Purchase_order_details (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            purchase_order_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity REAL NOT NULL,
            unit_cost REAL NOT NULL,
            unit_cost_mxn REAL NOT NULL DEFAULT 0,
            applied_unit_cost_mxn REAL NOT NULL DEFAULT 0,
            applied_total_mxn REAL NOT NULL DEFAULT 0,
            unit_purchase TEXT DEFAULT NULL,
            unit_quantity INTEGER DEFAULT NULL,
            unit_sale TEXT DEFAULT NULL,
            FOREIGN KEY (purchase_order_id) REFERENCES Purchase_orders(object_id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES Product(object_id) ON DELETE CASCADE
        );
CREATE TRIGGER update_Purchase_order_details AFTER UPDATE ON Purchase_order_details
    BEGIN
        UPDATE Purchase_order_details SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_pod_order ON Purchase_order_details(purchase_order_id);
CREATE INDEX idx_pod_product ON Purchase_order_details(product_id);
CREATE TABLE Product_cost (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            product_id INTEGER NOT NULL,
            source_type TEXT NOT NULL DEFAULT 'PURCHASE_ORDER',
            source_id INTEGER,
            old_cost REAL,
            new_cost REAL NOT NULL,
            currency TEXT NOT NULL DEFAULT 'MXN',
            note TEXT,
            FOREIGN KEY (product_id) REFERENCES Product(object_id) ON DELETE CASCADE
        );
CREATE TRIGGER update_Product_cost AFTER UPDATE ON Product_cost
    BEGIN
        UPDATE Product_cost SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_product_cost_product ON Product_cost(product_id);
CREATE INDEX idx_product_cost_source ON Product_cost(source_type, source_id);
CREATE INDEX idx_product_cost_created_at ON Product_cost(created_at);
CREATE TABLE CatalogProduct (
            object_id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            code TEXT,
            name TEXT NOT NULL,
            department TEXT,
            unit TEXT,
            image_url TEXT
        );
CREATE TRIGGER update_CatalogProduct AFTER UPDATE ON CatalogProduct
    BEGIN
        UPDATE CatalogProduct SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_catalog_product_code ON CatalogProduct(code);
CREATE INDEX idx_catalog_product_name ON CatalogProduct(name);
CREATE INDEX idx_catalog_product_department ON CatalogProduct(department);
CREATE TRIGGER product_delete_cleanup_option_assignment
                BEFORE DELETE ON Product
                BEGIN
                    DELETE FROM Product_option_assignment WHERE product_id = OLD.object_id;
                END;
CREATE TABLE IF NOT EXISTS "Cash_movements" (
                    object_id INTEGER PRIMARY KEY,
                    created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                    updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
                    box_id INTEGER NULL,
                    user_id INTEGER NULL,
                    type TEXT NOT NULL,
                    amount REAL NOT NULL,
                    currency TEXT NOT NULL DEFAULT 'MXN',
                    note TEXT NULL,
                    metadata TEXT NULL,
                    tip REAL NOT NULL DEFAULT 0,
                    FOREIGN KEY (user_id) REFERENCES User(object_id) ON DELETE SET NULL,
                    FOREIGN KEY (box_id) REFERENCES Multibox(object_id) ON DELETE SET NULL
                );
CREATE TRIGGER update_Cash_movements AFTER UPDATE ON Cash_movements
    BEGIN
        UPDATE Cash_movements SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE TRIGGER multibox_delete_cash_movements
                  BEFORE DELETE ON Multibox
                      BEGIN
                          UPDATE Cash_movements SET box_id = 1 WHERE box_id = OLD.object_id AND OLD.object_id != 1;
                      END;
CREATE INDEX idx_cash_movements_register ON Cash_movements(box_id);
CREATE INDEX idx_cash_movements_createdAt ON Cash_movements(created_at);
CREATE TABLE Quotation_detail_options (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            quotation_detail_id INTEGER NOT NULL,
            product_option_id INTEGER NOT NULL,
            product_option_detail_id INTEGER NOT NULL,
            name_snapshot TEXT NOT NULL,
            price_delta_snapshot REAL NOT NULL DEFAULT 0
        );
CREATE TRIGGER update_Quotation_detail_options AFTER UPDATE ON Quotation_detail_options
    BEGIN
        UPDATE Quotation_detail_options SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_quotation_detail_options_detail ON Quotation_detail_options(quotation_detail_id);
CREATE TABLE Inventory_movements (
            object_id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            inventory_id INTEGER,
            product_id INTEGER NOT NULL,
            warehouse_id INTEGER,
            user_id INTEGER,
            qty REAL NOT NULL,
            qty_before REAL NOT NULL,
            qty_after REAL NOT NULL,
            movement_type TEXT NOT NULL,
            movement_direction TEXT NOT NULL,
            reference_type TEXT,
            reference_id INTEGER,
            source TEXT,
            notes TEXT,
            FOREIGN KEY (inventory_id) REFERENCES Inventory(object_id) ON DELETE SET NULL,
            FOREIGN KEY (product_id) REFERENCES Product(object_id) ON DELETE CASCADE,
            FOREIGN KEY (warehouse_id) REFERENCES Warehouse(object_id) ON DELETE SET NULL,
            FOREIGN KEY (user_id) REFERENCES User(object_id) ON DELETE SET NULL
        );
CREATE TRIGGER update_Inventory_movements AFTER UPDATE ON Inventory_movements
    BEGIN
        UPDATE Inventory_movements SET updated_at= DATETIME('NOW') WHERE ROWID = NEW.ROWID;
    END;
CREATE INDEX idx_inventory_movements_product ON Inventory_movements(product_id);
CREATE INDEX idx_inventory_movements_created ON Inventory_movements(created_at);
CREATE INDEX idx_inventory_movements_type ON Inventory_movements(movement_type);
CREATE INDEX idx_inventory_movements_direction ON Inventory_movements(movement_direction);
CREATE INDEX idx_inventory_movements_reference ON Inventory_movements(reference_type, reference_id);
CREATE INDEX idx_inventory_movements_warehouse ON Inventory_movements(warehouse_id);