package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/2c2bhdhw9z-cell/CultivationMMO/server/internal/skeletonhttp"
)

const version = "0.0.1-skeleton"

func main() {
	listen := flag.String("listen", "127.0.0.1:8080", "loopback listen address")
	selfCheck := flag.Bool("self-check", false, "validate locked skeleton behavior and exit")
	flag.Parse()

	config := skeletonhttp.Config{
		Component:      "account-api",
		Version:        version,
		ListenAddress:  *listen,
		NotReadyReason: "identity_and_database_not_configured",
	}
	if *selfCheck {
		if err := skeletonhttp.SelfCheck(config); err != nil {
			log.Fatalf("API_SELF_CHECK_FAILED: %v", err)
		}
		fmt.Println("API_SELF_CHECK_OK ready=false listener=loopback-only")
		return
	}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	fmt.Printf("API_SKELETON_STARTING listen=%s ready=false\n", *listen)
	if err := skeletonhttp.Run(ctx, config); err != nil {
		log.Fatalf("account API stopped: %v", err)
	}
}
