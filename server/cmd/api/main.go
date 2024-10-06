package main

import (
	"log"

	"github.com/db-keli/shinobi/internal/db"
	"github.com/db-keli/shinobi/internal/env"
	"github.com/db-keli/shinobi/internal/store"
)

func main() {
	cfg := config{
		addr: env.GetString("ADDR", ":8080"),
		db: dbConfig{
			addr:         env.GetString("DB_ADDR", "postgres://admin:adminpassword@localhost:5543/social?sslmode=disable"),
			maxOpenConns: env.GetInt("DB_MAX_OPEN_CONNS", 5),
			maxIdleConns: env.GetInt("DB_MAX_IDLE_CONNS", 2),
			MaxIdleTime:  env.GetString("DB_MAX_IDLE_TIME", "15m"),
		},
	}

	db, err := db.New(cfg.db.addr, cfg.db.maxOpenConns, cfg.db.maxIdleConns, cfg.db.MaxIdleTime)

	if err != nil {
		log.Fatalf("db.New: %v", err)
	}

	store := store.NewPostgresStorage(db)

	app := application{
		config: cfg,
		store:  store,
	}

	mux := app.mount()

	log.Fatal(app.run(mux))
}