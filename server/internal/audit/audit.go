package audit

import (
	"context"
	"errors"
	"time"
)

var ErrUnavailable = errors.New("durable audit sink is unavailable")

type Event struct {
	Action        string
	Actor         string
	CorrelationID string
	Environment   string
	OccurredAt    time.Time
	Outcome       string
	Reason        string
	Service       string
	Target        string
}

type Sink interface {
	Append(context.Context, Event) error
	Health(context.Context) error
}

// UnavailableSink is the only Task 2 implementation. It deliberately prevents
// a local skeleton from treating in-memory or gameplay-database logs as durable audit.
type UnavailableSink struct{}

func (UnavailableSink) Append(context.Context, Event) error {
	return ErrUnavailable
}

func (UnavailableSink) Health(context.Context) error {
	return ErrUnavailable
}
