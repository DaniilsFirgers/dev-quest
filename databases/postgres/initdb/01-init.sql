-- Create a test table flats
-- By default the columsn are NULL, if not specified otherwise, or default is set
-- use a composite type for address
CREATE TABLE IF NOT EXISTS flats(
    flat_id VARCHAR(255) PRIMARY KEY, -- VARCHAR is a string with a maximum length of 255 characters
    district VARCHAR(100) NOT NULL, -- VARCHAR is a shorthand for VARCHAR(255), which is a string with a maximum length of 255 characters
    street VARCHAR(150) NOT NULL,
    rooms SMALLINT NOT NULL, -- SMALLINT is a type for small integers
    floors_total SMALLINT NOT NULL, -- how many floors are in the building
    floor SMALLINT NOT NULL, -- on which floor the flat is located
    price FLOAT NOT NULL, -- FLOAT can make rounding errors, but we will use it
    area DECIMAL(5, 2) NOT NULL, -- DECIMAL is a type for numbers with a fixed number of digits before and after the decimal point
    short_description TEXT NOT NULL, -- TEXT is a type for long strings, unlimited length
    updated_at TIMESTAMPTZ DEFAULT NOW(), -- TIMESTAMPTZ is a type for timestamps with time zone
    created_at TIMESTAMPTZ DEFAULT NOW(), -- also can use CURRENT_TIMESTAMP for default value
    picture BYTEA, -- BLOB is a type for binary large objects
    is_filtered BOOLEAN DEFAULT FALSE -- BOOLEAN is a type for true/false values
);

-- create a link table to trace relationships between flat adds that were updated
CREATE TABLE IF NOT EXISTS flat_updates(
    flat_1_id VARCHAR(255) NOT NULL,
    flat_2_id VARCHAR(255) NOT NULL,
    PRIMARY KEY(flat_1_id, flat_2_id),
    FOREIGN KEY(flat_1_id) REFERENCES flats(flat_id) ON DELETE CASCADE, -- foreign key is a reference to another table
    FOREIGN KEY(flat_2_id) REFERENCES flats(flat_id) ON DELETE CASCADE -- ON DELETE CASCADE means that if a flat is deleted, all references to it will be deleted as well
)

-- create indexes
CREATE INDEX idx_filtered_flats ON flats(is_filtered);
CREATE INDEX idx_district ON flats(district);
-- create composite index
CREATE INDEX idx_district_price ON flats(district, price);
-- create constraints
ALTER TABLE flats ADD CONSTRAINT chk_price CHECK (price > 0);
ALTER TABLE flats ADD CONSTRAINT chk_area CHECK (area > 0);
ALTER TABLE flats ADD CONSTRAINT chk_floor CHECK (floor <= floors_total);
ALTER TABLE flats ADD CONSTRAINT chk_rooms CHECK (rooms > 0);