env "local" {
  src = "file://atlas/schema.hcl"

  // kalau yang ku dapat, kayak production database
  url = "postgres://root:root@192.168.1.101:5434/quotes?search_path=public&sslmode=disable"

  // ini url utuk database dev atlas
  dev = "docker://postgres/16/dev?search_path=public"

  // setup folder migrasi
  migration {
    dir = "file://migrations"
  }
}

env "test" {
  src = "file://atlas/schema.hcl"

  // kalau yang ku dapat, kayak production database
  url = "postgres://root:root@192.168.1.101:5434/quotes_test?search_path=public&sslmode=disable"

  // ini url utuk database dev atlas
  dev = "docker://postgres/16/dev?search_path=public"

  // setup folder migrasi
  migration {
    dir = "file://migrations"
  }
}

env "prod" {
  src = "file://atlas/schema.hcl"

  // kalau yang ku dapat, kayak production database
  url = "postgres://root:root@192.168.1.101:5434/quotes_prod?search_path=public&sslmode=disable"

  // ini url utuk database dev atlas
  dev = "docker://postgres/16/dev?search_path=public"

  // setup folder migrasi
  migration {
    dir = "file://migrations"
  }
}