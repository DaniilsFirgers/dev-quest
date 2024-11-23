-- Create a test table flats
-- By default the columsn are NULL, if not specified otherwise, or default is set
CREATE TABLE IF NOT EXISTS flats(
    flat_id VARCHAR(255) PRIMARY KEY, -- VARCHAR is a string with a maximum length of 255 characters
    address VARCHAR(255) NOT NULL, -- VARCHAR is a shorthand for VARCHAR(255), which is a string with a maximum length of 255 characters
    price FLOAT NOT NULL, -- FLOAT can make rounding errors, but we will use it
    rooms SMALLINT NOT NULL, -- SMALLINT is a type for small integers
    area DECIMAL(4, 2) NOT NULL, -- DECIMAL is a type for numbers with a fixed number of digits before and after the decimal point
    description TEXT NOT NULL -- TEXT is a type for long strings, unlimited length
    updated_at TIMESTAMPTZ DEFAULT NOW() -- TIMESTAMPTZ is a type for timestamps with time zone
    created_at TIMESTAMPTZ DEFAULT NOW() -- also can use CURRENT_TIMESTAMP for default value
    picture BLOB -- BLOB is a type for binary large objects
    is_filtered BOOLEAN DEFAULT FALSE -- BOOLEAN is a type for true/false values
)

-- create a link table to trace relationships between flat adds that were updated
CREATE TABLE IF NOT EXISTS flat_updates(
    flat_1_id VARCHAR(255) NOT NULL,
    flat_2_id VARCHAR(255) NOT NULL,
    PRIMARY KEY(flat_1_id, flat_2_id),
    FOREIGN KEY(flat_1_id) REFERENCES flats(flat_id) ON DELETE CASCADE, -- foreign key is a reference to another table
    FOREIGN KEY(flat_2_id) REFERENCES flats(flat_id) ON DELETE CASCADE -- ON DELETE CASCADE means that if a flat is deleted, all references to it will be deleted as well
)