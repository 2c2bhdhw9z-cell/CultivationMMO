package contracts

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"reflect"
	"strconv"
	"strings"
	"unicode/utf8"
)

const maxContractBytes = 1024 * 1024

type Registry struct {
	Commands []CommandDefinition `json:"commands"`
	Protocol int                 `json:"protocol"`
	Status   string              `json:"status"`
}

type CommandDefinition struct {
	Action            string `json:"action"`
	CommandID         string `json:"command_id"`
	FeatureID         string `json:"feature_id"`
	OwningService     string `json:"owning_service"`
	RequiresFreshAuth bool   `json:"requires_fresh_auth"`
}

type OperationsManifest struct {
	Features      []FeatureOperations `json:"features"`
	SchemaVersion int                 `json:"schema_version"`
}

type FeatureOperations struct {
	Audit       Control `json:"audit"`
	Authorize   Control `json:"authorization"`
	Configure   Control `json:"configure"`
	Diagnose    Control `json:"diagnose"`
	FeatureID   string  `json:"feature_id"`
	Inspect     Control `json:"inspect"`
	PhoneUI     Control `json:"phone_ui"`
	RepairReset Control `json:"repair_reset"`
	Verify      Control `json:"verify"`
}

type Control struct {
	Method string `json:"method"`
	Reason string `json:"reason,omitempty"`
	Status string `json:"status"`
}

func LoadRegistry(path string) (Registry, error) {
	var registry Registry
	if err := LoadStrictJSON(path, &registry); err != nil {
		return Registry{}, err
	}
	if registry.Protocol != 1 {
		return Registry{}, errors.New("owner command registry protocol must be 1")
	}
	if registry.Status != "locked" {
		return Registry{}, errors.New("Task 2 owner command registry must remain locked")
	}
	seen := map[string]struct{}{}
	for _, command := range registry.Commands {
		if !validIdentifier(command.CommandID, 128) {
			return Registry{}, errors.New("owner command ID is invalid")
		}
		if !validIdentifier(command.FeatureID, 128) {
			return Registry{}, errors.New("owner command feature ID is invalid")
		}
		if !validIdentifier(command.OwningService, 96) || !validIdentifier(command.Action, 96) {
			return Registry{}, errors.New("owner command service or action is invalid")
		}
		if _, exists := seen[command.CommandID]; exists {
			return Registry{}, fmt.Errorf("duplicate owner command: %s", command.CommandID)
		}
		seen[command.CommandID] = struct{}{}
	}
	return registry, nil
}

func LoadOperations(path string) (OperationsManifest, error) {
	var manifest OperationsManifest
	if err := LoadStrictJSON(path, &manifest); err != nil {
		return OperationsManifest{}, err
	}
	if err := ValidateOperations(manifest); err != nil {
		return OperationsManifest{}, err
	}
	return manifest, nil
}

func ValidateOperations(manifest OperationsManifest) error {
	if manifest.SchemaVersion != 1 || len(manifest.Features) == 0 {
		return errors.New("operations manifest must contain V1 features")
	}
	seen := map[string]struct{}{}
	for _, feature := range manifest.Features {
		if !validIdentifier(feature.FeatureID, 128) {
			return errors.New("operations feature ID is invalid")
		}
		if _, exists := seen[feature.FeatureID]; exists {
			return fmt.Errorf("duplicate operations feature: %s", feature.FeatureID)
		}
		seen[feature.FeatureID] = struct{}{}
		controls := []Control{
			feature.Inspect,
			feature.Diagnose,
			feature.Configure,
			feature.RepairReset,
			feature.PhoneUI,
			feature.Authorize,
			feature.Audit,
			feature.Verify,
		}
		for _, control := range controls {
			if err := validateControl(feature.FeatureID, control); err != nil {
				return err
			}
		}
	}
	return nil
}

func ValidateRegistryCoverage(registry Registry, manifest OperationsManifest) error {
	features := map[string]struct{}{}
	for _, feature := range manifest.Features {
		features[feature.FeatureID] = struct{}{}
	}
	for _, command := range registry.Commands {
		if _, exists := features[command.FeatureID]; !exists {
			return fmt.Errorf("command %s references missing operations feature %s", command.CommandID, command.FeatureID)
		}
	}
	return nil
}

func LoadStrictJSON(path string, target any) error {
	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("read %s: %w", path, err)
	}
	if err := DecodeStrictJSON(data, target); err != nil {
		return fmt.Errorf("decode %s: %w", path, err)
	}
	return nil
}

func DecodeStrictJSON(data []byte, target any) error {
	if target == nil || reflect.TypeOf(target).Kind() != reflect.Pointer || reflect.ValueOf(target).IsNil() {
		return errors.New("strict JSON target must be a non-nil pointer")
	}
	if len(data) == 0 {
		return errors.New("JSON is empty")
	}
	if len(data) > maxContractBytes {
		return errors.New("JSON exceeds 1 MiB limit")
	}
	if !utf8.Valid(data) {
		return errors.New("JSON is not valid UTF-8")
	}
	if err := rejectDuplicateKeys(data); err != nil {
		return err
	}

	var shape any
	shapeDecoder := json.NewDecoder(bytes.NewReader(data))
	shapeDecoder.UseNumber()
	if err := shapeDecoder.Decode(&shape); err != nil {
		return err
	}
	if err := validateExactShape(shape, reflect.TypeOf(target).Elem(), "$"); err != nil {
		return err
	}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	decoder.UseNumber()
	if err := decoder.Decode(target); err != nil {
		return err
	}
	var extra any
	if err := decoder.Decode(&extra); !errors.Is(err, io.EOF) {
		if err == nil {
			return errors.New("extra JSON content")
		}
		return fmt.Errorf("trailing JSON data: %w", err)
	}
	return nil
}

func validateExactShape(value any, targetType reflect.Type, path string) error {
	for targetType.Kind() == reflect.Pointer {
		targetType = targetType.Elem()
	}
	if value == nil {
		return fmt.Errorf("%s: required value must not be null", path)
	}

	switch targetType.Kind() {
	case reflect.Struct:
		object, ok := value.(map[string]any)
		if !ok {
			return fmt.Errorf("%s: expected object", path)
		}
		type fieldInfo struct {
			required bool
			typeOf   reflect.Type
		}
		fields := map[string]fieldInfo{}
		for index := 0; index < targetType.NumField(); index++ {
			field := targetType.Field(index)
			if field.PkgPath != "" {
				continue
			}
			tag := field.Tag.Get("json")
			if tag == "-" {
				continue
			}
			parts := strings.Split(tag, ",")
			name := parts[0]
			if name == "" {
				name = field.Name
			}
			required := true
			for _, option := range parts[1:] {
				if option == "omitempty" {
					required = false
				}
			}
			fields[name] = fieldInfo{required: required, typeOf: field.Type}
		}
		for key := range object {
			if _, exists := fields[key]; !exists {
				return fmt.Errorf("%s: unknown exact field %q", path, key)
			}
		}
		for name, field := range fields {
			child, exists := object[name]
			if !exists {
				if field.required {
					return fmt.Errorf("%s: required exact field %q is missing", path, name)
				}
				continue
			}
			if err := validateExactShape(child, field.typeOf, path+"."+name); err != nil {
				return err
			}
		}
	case reflect.Slice, reflect.Array:
		array, ok := value.([]any)
		if !ok {
			return fmt.Errorf("%s: expected array", path)
		}
		for index, child := range array {
			if err := validateExactShape(child, targetType.Elem(), fmt.Sprintf("%s[%d]", path, index)); err != nil {
				return err
			}
		}
	case reflect.Interface:
		return nil
	case reflect.String:
		if _, ok := value.(string); !ok {
			return fmt.Errorf("%s: expected string", path)
		}
	case reflect.Bool:
		if _, ok := value.(bool); !ok {
			return fmt.Errorf("%s: expected boolean", path)
		}
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		number, ok := value.(json.Number)
		if !ok {
			return fmt.Errorf("%s: expected integer", path)
		}
		if _, err := strconv.ParseInt(number.String(), 10, targetType.Bits()); err != nil {
			return fmt.Errorf("%s: invalid integer: %w", path, err)
		}
	}
	return nil
}

func rejectDuplicateKeys(data []byte) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.UseNumber()
	if err := consumeValue(decoder, "$"); err != nil {
		return err
	}
	if _, err := decoder.Token(); !errors.Is(err, io.EOF) {
		if err == nil {
			return errors.New("extra JSON value")
		}
		return fmt.Errorf("trailing JSON token: %w", err)
	}
	return nil
}

func consumeValue(decoder *json.Decoder, path string) error {
	token, err := decoder.Token()
	if err != nil {
		return fmt.Errorf("%s: %w", path, err)
	}
	delimiter, isDelimiter := token.(json.Delim)
	if !isDelimiter {
		return nil
	}

	switch delimiter {
	case '{':
		seen := map[string]struct{}{}
		for decoder.More() {
			keyToken, err := decoder.Token()
			if err != nil {
				return fmt.Errorf("%s: object key: %w", path, err)
			}
			key, ok := keyToken.(string)
			if !ok {
				return fmt.Errorf("%s: object key is not a string", path)
			}
			if _, exists := seen[key]; exists {
				return fmt.Errorf("%s: duplicate key %q", path, key)
			}
			seen[key] = struct{}{}
			if err := consumeValue(decoder, path+"."+key); err != nil {
				return err
			}
		}
		closing, err := decoder.Token()
		if err != nil || closing != json.Delim('}') {
			return fmt.Errorf("%s: object is not closed", path)
		}
	case '[':
		index := 0
		for decoder.More() {
			if err := consumeValue(decoder, fmt.Sprintf("%s[%d]", path, index)); err != nil {
				return err
			}
			index++
		}
		closing, err := decoder.Token()
		if err != nil || closing != json.Delim(']') {
			return fmt.Errorf("%s: array is not closed", path)
		}
	default:
		return fmt.Errorf("%s: unexpected delimiter %q", path, delimiter)
	}
	return nil
}

func validateControl(featureID string, control Control) error {
	switch control.Status {
	case "local":
		if control.Method == "" {
			return fmt.Errorf("%s local control method is required", featureID)
		}
		if control.Reason != "" {
			return fmt.Errorf("%s local control must not include a reason", featureID)
		}
	case "not_applicable":
		if control.Method != "" {
			return fmt.Errorf("%s not-applicable control must not claim a method", featureID)
		}
		if control.Reason == "" {
			return fmt.Errorf("%s not-applicable control needs a reason", featureID)
		}
	default:
		return fmt.Errorf("%s has invalid control status %q", featureID, control.Status)
	}
	return nil
}

func validIdentifier(value string, maxBytes int) bool {
	if value == "" || len(value) > maxBytes {
		return false
	}
	for _, character := range value {
		if (character >= 'a' && character <= 'z') ||
			(character >= 'A' && character <= 'Z') ||
			(character >= '0' && character <= '9') ||
			strings.ContainsRune("-_.:", character) {
			continue
		}
		return false
	}
	return true
}
