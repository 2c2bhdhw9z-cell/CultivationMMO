package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/audit"
	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/contracts"
	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/skeletonhttp"
)

const version = "0.0.1-skeleton"

func main() {
	listen := flag.String("listen", "127.0.0.1:8081", "loopback listen address")
	registryPath := flag.String("registry", "../shared/owner-control/v1/registry.json", "owner command registry")
	operationsPath := flag.String("operations", "../shared/operations/v1/features.json", "feature operations manifest")
	selfCheck := flag.Bool("self-check", false, "validate locked skeleton behavior and exit")
	flag.Parse()

	registry, err := contracts.LoadRegistry(*registryPath)
	if err != nil {
		log.Fatalf("owner registry invalid: %v", err)
	}
	operations, err := contracts.LoadOperations(*operationsPath)
	if err != nil {
		log.Fatalf("operations manifest invalid: %v", err)
	}
	if err := contracts.ValidateRegistryCoverage(registry, operations); err != nil {
		log.Fatalf("registry coverage invalid: %v", err)
	}
	if len(registry.Commands) != 0 {
		log.Fatal("Task 2 must not expose a privileged command")
	}

	config := skeletonhttp.Config{
		Component:      "owner-control",
		Version:        version,
		ListenAddress:  *listen,
		NotReadyReason: "owner_identity_and_durable_audit_not_configured",
	}
	if *selfCheck {
		if err := skeletonhttp.SelfCheck(config); err != nil {
			log.Fatalf("OWNER_CONTROL_SELF_CHECK_FAILED: %v", err)
		}
		invalidRegistryDocuments := []string{
			`{"protocol":1,"protocol":1,"status":"locked","commands":[]}`,
			`{"protocol":1,"\u0070rotocol":1,"status":"locked","commands":[]}`,
			`{"Protocol":1,"status":"locked","commands":[]}`,
			`{"protocol":1,"status":"locked","commands":[],"Commands":[]}`,
			`{"protocol":1,"status":"locked","commands":[],"unknown":true}`,
			`{"protocol":1,"status":"locked"}`,
			`{"protocol":1,"status":"locked","commands":null}`,
			`{"protocol":1,"status":"locked","commands":[{"command_id":"one","feature_id":"skeleton.owner_control","owning_service":"owner-control","action":"inspect"}]}`,
			`{"protocol":1,"status":"locked","commands":[{"command_id":"one","feature_id":"skeleton.owner_control","owning_service":"owner-control","action":"inspect","requires_fresh_auth":null}]}`,
		}
		for _, document := range invalidRegistryDocuments {
			var invalidRegistry contracts.Registry
			if err := contracts.DecodeStrictJSON([]byte(document), &invalidRegistry); err == nil {
				log.Fatal("OWNER_CONTROL_SELF_CHECK_FAILED: non-canonical registry JSON was accepted")
			}
		}
		var nullControl contracts.Control
		if err := contracts.DecodeStrictJSON(
			[]byte(`{"status":"not_applicable","method":null,"reason":"required"}`),
			&nullControl,
		); err == nil {
			log.Fatal("OWNER_CONTROL_SELF_CHECK_FAILED: null scalar was accepted")
		}
		invalidOperations := operations
		invalidOperations.Features = append([]contracts.FeatureOperations(nil), operations.Features...)
		invalidOperations.Features[0].Inspect.Reason = "local controls must not include a reason"
		if err := contracts.ValidateOperations(invalidOperations); err == nil {
			log.Fatal("OWNER_CONTROL_SELF_CHECK_FAILED: schema-invalid local control was accepted")
		}
		sink := audit.UnavailableSink{}
		if err := sink.Health(context.Background()); !errors.Is(err, audit.ErrUnavailable) {
			log.Fatal("OWNER_CONTROL_SELF_CHECK_FAILED: audit health did not fail closed")
		}
		if err := sink.Append(context.Background(), audit.Event{}); !errors.Is(err, audit.ErrUnavailable) {
			log.Fatal("OWNER_CONTROL_SELF_CHECK_FAILED: audit append did not fail closed")
		}
		fmt.Printf("OWNER_CONTROL_SELF_CHECK_OK commands=%d features=%d ready=false audit=unavailable strict-json=true\n", len(registry.Commands), len(operations.Features))
		return
	}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	fmt.Printf("OWNER_CONTROL_SKELETON_STARTING listen=%s commands=0 protected-assets=unserved\n", *listen)
	if err := skeletonhttp.Run(ctx, config); err != nil {
		log.Fatalf("owner control stopped: %v", err)
	}
}
