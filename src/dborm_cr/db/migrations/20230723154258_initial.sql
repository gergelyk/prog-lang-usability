-- +micrate Up
CREATE TABLE stock(name VARCHAR PRIMARY KEY NOT NULL,
                   vendor VARCHAR NOT NULL,
                   quantity INT);

-- +micrate Down
DROP TABLE stock;
