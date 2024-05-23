
CREATE TABLE staff_users (
    user_id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    mob_phone VARCHAR(50) UNIQUE NOT NULL,
    passwd VARCHAR(100) NOT NULL,
    acc_level VARCHAR(20) DEFAULT 'trainee' NOT NULL,
    status VARCHAR(20) DEFAULT 'active' NOT NULL CHECK (status IN ('active', 'suspended')),
    a_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT idx_username UNIQUE (username),
    CONSTRAINT idx_mob_phone UNIQUE (mob_phone)
);

CREATE TABLE diary_settings (
    staff_id UUID PRIMARY KEY,
    diary_colour VARCHAR(20),
    popup_notifi_en BOOLEAN,
    email_notifi_en BOOLEAN,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (staff_id) REFERENCES staff_users(user_id) ON DELETE CASCADE
);

CREATE TABLE address (
    address_id UUID PRIMARY KEY,
    staff_id UUID REFERENCES staff_users(user_id) ON DELETE CASCADE,
    address_line_1 VARCHAR(255) NOT NULL,
    address_line_2 VARCHAR(255),
    town_city VARCHAR(255) NOT NULL,
    county VARCHAR(255),
    postcode VARCHAR(10) NOT NULL,
    country VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE TABLE audit_trail (
    entry_id UUID PRIMARY KEY,
    user_id UUID REFERENCES staff_users(user_id) ON DELETE CASCADE,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    description VARCHAR(255) NOT NULL,
    action_type VARCHAR(20) NOT NULL,
    ip_address VARCHAR(15)
);

CREATE TABLE changes_made (
    entry_id UUID PRIMARY KEY,
    user_id UUID REFERENCES staff_users(user_id) ON DELETE CASCADE,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    description VARCHAR(255) NOT NULL,
    action_type VARCHAR(20) NOT NULL,
    target_user VARCHAR(100)
);

CREATE TABLE notes (
    note_id UUID PRIMARY KEY,
    user_id UUID REFERENCES staff_users(user_id) ON DELETE CASCADE,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    note_text TEXT NOT NULL
);
