CREATE TABLE companies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(name) BETWEEN 1 AND 80),
    address TEXT NOT NULL CHECK(length(address) BETWEEN 1 AND 80),
    contact_no TEXT NOT NULL CHECK(length(contact_no) BETWEEN 1 AND 20),
    is_active INTEGER DEFAULT 1,
    create_date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK(length(name) BETWEEN 1 AND 80),
    contact_no TEXT NOT NULL CHECK(length(contact_no) BETWEEN 1 AND 20),
    email TEXT NOT NULL CHECK(length(email) BETWEEN 1 AND 80),
    comp_id INTEGER REFERENCES companies(id) ON DELETE CASCADE,
    address_id INTEGER,
    title_id INTEGER,
    gender_id INTEGER,
    is_active INTEGER DEFAULT 1,
    create_date TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX employees_company ON employees(comp_id);

INSERT INTO companies (name, address, contact_no) VALUES
    ('Cedar Grove Studio', '125 Cedar Lane, Brookfield', '202-555-0142'),
    ('Harborline Analytics', '480 Harbor Way, Lakehaven', '202-555-0176');

INSERT INTO employees (name, contact_no, email, comp_id) VALUES
    ('Maya Chen', '202-555-0143', 'maya.chen@cedargrove.test', 1),
    ('Jordan Rivera', '202-555-0177', 'jordan.rivera@harborline.test', 2);

