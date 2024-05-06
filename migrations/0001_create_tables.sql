CREATE TABLE customer_nwc (
  id INT GENERATED ALWAYS AS IDENTITY NOT NULL,
  uuid VARCHAR NOT NULL,
  server_key VARCHAR NOT NULL,
  user_key VARCHAR NOT NULL,
  uri VARCHAR NOT NULL UNIQUE,
  app_service VARCHAR NOT NULL,
  budget BIGINT NOT NULL,
  PRIMARY KEY(id)
);

ALTER TABLE customer_nwc
ADD UNIQUE (uuid, app_service);
