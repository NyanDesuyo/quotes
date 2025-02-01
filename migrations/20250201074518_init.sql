-- Create "quotes" table
CREATE TABLE "quotes" (
    "id" bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    "uuid" uuid NOT NULL,
    "book" character varying NOT NULL,
    "quote" text NOT NULL,
    "remarks" text NULL,
    "inserted_at" timestamptz NOT NULL,
    "updated_at" timestamptz NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT "quotes_book_quote_key" UNIQUE ("book", "quote")
);