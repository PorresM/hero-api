-- Your SQL goes here
CREATE TABLE hero (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    identity VARCHAR NOT NULL,
    hometown VARCHAR NOT NULL,
    age INTEGER NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_modified_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.modified = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_hero_modified BEFORE UPDATE ON hero FOR EACH ROW EXECUTE PROCEDURE update_modified_column();