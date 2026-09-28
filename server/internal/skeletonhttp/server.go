package skeletonhttp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net"
	"net/http"
	"net/http/httptest"
	"strings"
	"time"
)

const ProtocolVersion = 1

type Config struct {
	Component      string
	Version        string
	ListenAddress  string
	NotReadyReason string
}

type statusResponse struct {
	Component string `json:"component"`
	Protocol  int    `json:"protocol"`
	Ready     bool   `json:"ready"`
	Reason    string `json:"reason,omitempty"`
	Status    string `json:"status"`
	Version   string `json:"version"`
}

func Handler(config Config) http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /healthz", func(writer http.ResponseWriter, _ *http.Request) {
		writeJSON(writer, http.StatusOK, statusResponse{
			Component: config.Component,
			Protocol:  ProtocolVersion,
			Ready:     false,
			Status:    "skeleton",
			Version:   config.Version,
		})
	})
	mux.HandleFunc("GET /readyz", func(writer http.ResponseWriter, _ *http.Request) {
		writeJSON(writer, http.StatusServiceUnavailable, statusResponse{
			Component: config.Component,
			Protocol:  ProtocolVersion,
			Ready:     false,
			Reason:    config.NotReadyReason,
			Status:    "locked",
			Version:   config.Version,
		})
	})
	mux.HandleFunc("/", func(writer http.ResponseWriter, _ *http.Request) {
		http.Error(writer, "not found", http.StatusNotFound)
	})
	return securityHeaders(mux)
}

func Run(ctx context.Context, config Config) error {
	if err := validate(config); err != nil {
		return err
	}
	if err := ctx.Err(); err != nil {
		return nil
	}
	listener, err := net.Listen("tcp", config.ListenAddress)
	if err != nil {
		return fmt.Errorf("listen: %w", err)
	}

	server := &http.Server{
		Addr:              config.ListenAddress,
		Handler:           Handler(config),
		ReadHeaderTimeout: 3 * time.Second,
		IdleTimeout:       30 * time.Second,
		MaxHeaderBytes:    8 * 1024,
	}

	go func() {
		<-ctx.Done()
		shutdownContext, cancel := context.WithTimeout(context.Background(), 3*time.Second)
		defer cancel()
		_ = server.Shutdown(shutdownContext)
	}()

	err = server.Serve(listener)
	if err != nil && !errors.Is(err, http.ErrServerClosed) {
		return fmt.Errorf("serve: %w", err)
	}
	return nil
}

func SelfCheck(config Config) error {
	if err := validate(config); err != nil {
		return err
	}
	checks := []struct {
		method string
		path   string
		want   int
	}{
		{method: http.MethodGet, path: "/healthz", want: http.StatusOK},
		{method: http.MethodGet, path: "/readyz", want: http.StatusServiceUnavailable},
		{method: http.MethodGet, path: "/", want: http.StatusNotFound},
		{method: http.MethodPost, path: "/healthz", want: http.StatusNotFound},
		{method: http.MethodGet, path: "/owner", want: http.StatusNotFound},
	}
	for _, check := range checks {
		request := httptest.NewRequest(check.method, check.path, nil)
		response := httptest.NewRecorder()
		Handler(config).ServeHTTP(response, request)
		if response.Code != check.want {
			return fmt.Errorf("%s %s: got %d, want %d", check.method, check.path, response.Code, check.want)
		}
		if err := verifyHeaders(response.Header()); err != nil {
			return fmt.Errorf("%s %s: %w", check.method, check.path, err)
		}
		if check.method == http.MethodGet && (check.path == "/healthz" || check.path == "/readyz") {
			if !strings.HasPrefix(response.Header().Get("Content-Type"), "application/json") {
				return fmt.Errorf("%s: status response is not JSON", check.path)
			}
		}
		if check.path == "/healthz" && check.method == http.MethodGet {
			if err := verifyStatusBody(response.Body.Bytes(), config, "skeleton", "", false); err != nil {
				return err
			}
		}
		if check.path == "/readyz" {
			if err := verifyStatusBody(response.Body.Bytes(), config, "locked", config.NotReadyReason, false); err != nil {
				return err
			}
		}
		if check.want == http.StatusNotFound && strings.TrimSpace(response.Body.String()) != "not found" {
			return fmt.Errorf("%s %s: non-generic not-found body", check.method, check.path)
		}
	}
	return nil
}

func verifyHeaders(header http.Header) error {
	expected := map[string]string{
		"Cache-Control":           "no-store",
		"Content-Security-Policy": "default-src 'none'; frame-ancestors 'none'",
		"Referrer-Policy":         "no-referrer",
		"X-Content-Type-Options":  "nosniff",
		"X-Frame-Options":         "DENY",
	}
	for name, want := range expected {
		if got := header.Get(name); got != want {
			return fmt.Errorf("header %s: got %q, want %q", name, got, want)
		}
	}
	return nil
}

func verifyStatusBody(body []byte, config Config, status string, reason string, ready bool) error {
	var response statusResponse
	if err := json.Unmarshal(body, &response); err != nil {
		return fmt.Errorf("status response JSON: %w", err)
	}
	if response.Component != config.Component || response.Protocol != ProtocolVersion || response.Version != config.Version {
		return errors.New("status response identity mismatch")
	}
	if response.Status != status || response.Reason != reason || response.Ready != ready {
		return errors.New("status response state mismatch")
	}
	return nil
}

func validate(config Config) error {
	if config.Component == "" || config.Version == "" || config.NotReadyReason == "" {
		return errors.New("component, version, and not-ready reason are required")
	}
	if config.ListenAddress == "" || !isLoopback(config.ListenAddress) {
		return errors.New("skeleton services may bind only to an explicit loopback address")
	}
	return nil
}

func isLoopback(address string) bool {
	host, _, err := net.SplitHostPort(address)
	if err != nil {
		return false
	}
	if host == "localhost" {
		return true
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

func securityHeaders(next http.Handler) http.Handler {
	return http.HandlerFunc(func(writer http.ResponseWriter, request *http.Request) {
		writer.Header().Set("Cache-Control", "no-store")
		writer.Header().Set("Content-Security-Policy", "default-src 'none'; frame-ancestors 'none'")
		writer.Header().Set("Referrer-Policy", "no-referrer")
		writer.Header().Set("X-Content-Type-Options", "nosniff")
		writer.Header().Set("X-Frame-Options", "DENY")
		next.ServeHTTP(writer, request)
	})
}

func writeJSON(writer http.ResponseWriter, status int, value statusResponse) {
	writer.Header().Set("Content-Type", "application/json")
	writer.WriteHeader(status)
	_ = json.NewEncoder(writer).Encode(value)
}
