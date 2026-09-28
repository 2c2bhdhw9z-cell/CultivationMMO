package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/contracts"
)

type migrationManifest struct {
	Database      string `json:"database"`
	Migrations    []any  `json:"migrations"`
	SchemaVersion int    `json:"schema_version"`
	Status        string `json:"status"`
}

func main() {
	manifestPath := flag.String("manifest", "database/migrations/manifest.json", "migration manifest")
	selfCheck := flag.Bool("self-check", false, "verify unconfigured migration state and exit")
	flag.Parse()

	var manifest migrationManifest
	if err := contracts.LoadStrictJSON(*manifestPath, &manifest); err != nil {
		log.Fatalf("migration manifest invalid: %v", err)
	}
	if manifest.SchemaVersion != 1 || manifest.Status != "unconfigured" || len(manifest.Migrations) != 0 {
		log.Fatal("database skeleton must remain unconfigured with zero migrations")
	}
	if manifest.Database != "postgresql-version-unselected" {
		log.Fatal("database version must remain explicitly unselected in Task 2")
	}
	if *selfCheck {
		fmt.Println("DATABASE_SELF_CHECK_OK configured=false migrations=0 connection=none strict-json=true")
		return
	}

	fmt.Fprintln(os.Stderr, "database is not configured; no migration or connection was attempted")
	os.Exit(2)
}
