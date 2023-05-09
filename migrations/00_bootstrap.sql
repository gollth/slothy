CREATE TABLE Plant (
  iot            INTEGER  NOT NULL,
  sensor         INTEGER  NOT NULL,
  name           TEXT     NOT NULL,
  PRIMARY KEY    (iot, sensor)
);

CREATE TABLE Observation (
  id             INTEGER  UNIQUE NOT NULL,
  stamp          DATETIME NOT NULL DEFAULT (DATETIME('now')),
  plant          TEXT     NOT NULL,
  humidity       REAL     NOT NULL,
  PRIMARY KEY    (id AUTOINCREMENT)
);

CREATE TABLE Battery (
  id             INTEGER UNIQUE NOT NULL,
  iot            INTEGER NOT NULL,
  voltage        REAL    NOT NULL,
  PRIMARY KEY    (id AUTOINCREMENT)
);
