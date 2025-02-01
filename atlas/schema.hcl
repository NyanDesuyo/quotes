table "quotes" {
  schema = schema.public
  column "id" {
    null = false
    type = bigint
    identity {
      generated = ALWAYS
      start = 1
      increment = 1
    }
  }
  column "uuid" {
    null = false
    type = uuid
  }
  column "book" {
    null = false
    type = character_varying
  }
  column "quote" {
    null = false
    type = text
  }
  column "remarks" {
    null = true
    type = text
  }
  column "inserted_at" {
    null = false
    type = timestamptz
  }
  column "updated_at" {
    null = false
    type = timestamptz
  }
  primary_key {
    columns = [column.id]
  }
  unique "quotes_book_quote_key" {
    columns = [column.book, column.quote]
  }
}
schema "public" {
  comment = "standard public schema"
}
