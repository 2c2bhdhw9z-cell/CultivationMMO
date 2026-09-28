package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/contracts"
)

type planManifest struct {
	Plans         []any  `json:"plans"`
	SchemaVersion int    `json:"schema_version"`
	Status        string `json:"status"`
}

func main() {
	manifestPath := flag.String("manifest", "../shared/maintenance/v1/plans.json", "reviewed repair-plan manifest")
	selfCheck := flag.Bool("self-check", false, "verify dormant behavior and exit")
	flag.Parse()

	var manifest planManifest
	if err := contracts.LoadStrictJSON(*manifestPath, &manifest); err != nil {
		log.Fatalf("maintenance manifest invalid: %v", err)
	}
	if manifest.SchemaVersion != 1 || manifest.Status != "dormant" || len(manifest.Plans) != 0 {
		log.Fatal("maintenance skeleton must remain dormant with zero executable plans")
	}
	if *selfCheck {
		fmt.Println("MAINTENANCE_SELF_CHECK_OK plans=0 listener=none credentials=none strict-json=true")
		return
	}

	fmt.Fprintln(os.Stderr, "maintenance executor is dormant; no reviewed repair plan is registered")
	os.Exit(2)
}
