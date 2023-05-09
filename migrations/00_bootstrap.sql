CREATE TABLE Plant (
  id             INTEGER  UNIQUE,
  name           TEXT     NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE Water (
    id             INTEGER  UNIQUE,
    plant          INTEGER  NOT NULL,
    humidity       REAL     NOT NULL,
    stamp          DATETIME NOT NULL DEFAULT (DATETIME('now')),
    PRIMARY KEY (id AUTOINCREMENT)
);
