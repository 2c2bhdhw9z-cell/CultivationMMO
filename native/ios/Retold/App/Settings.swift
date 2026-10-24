import SwiftUI

/// Every setting the player can change. Defaults match the original game where there is one.
final class Settings: ObservableObject {
    static let shared = Settings()

    // Text
    @AppStorage("textSpeed") var textSpeed = 3            // 0 instant, 1 fast, 3 medium, 5 slow
    @AppStorage("holdToSpeedText") var holdToSpeedText = true
    // Moving
    @AppStorage("walkSpeed") var walkSpeed = 1            // pixels per frame; 1 is the original
    @AppStorage("runWithB") var runWithB = true
    @AppStorage("runSpeed") var runSpeed = 2
    // Look
    @AppStorage("colors") var colors = 0                  // 0 Super Game Boy, 1 classic green, 2 grey
    @AppStorage("fillScreen") var fillScreen = true
    @AppStorage("smoothPixels") var smoothPixels = false
    @AppStorage("fades") var fades = true
    // Controls
    @AppStorage("controlsSize") var controlsSize = 1.0
    @AppStorage("controlsOpacity") var controlsOpacity = 0.85
    @AppStorage("haptics") var haptics = true
    @AppStorage("showControlsWithController") var showControlsWithController = false

    /// Sends the engine's settings to a running game.
    func apply(to s: Session) {
        s.set(RETOLD_OPT_TEXT_SPEED, textSpeed)
        s.set(RETOLD_OPT_HOLD_TO_SPEED_TEXT, holdToSpeedText ? 1 : 0)
        s.set(RETOLD_OPT_WALK_SPEED, walkSpeed)
        s.set(RETOLD_OPT_RUN_WITH_B, runWithB ? 1 : 0)
        s.set(RETOLD_OPT_RUN_SPEED, runSpeed)
        s.set(RETOLD_OPT_COLORS, colors)
        s.set(RETOLD_OPT_FADES, fades ? 1 : 0)
    }
}

struct SettingsView: View {
    @ObservedObject var settings = Settings.shared
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            Form {
                Section("Text") {
                    Picker("Text speed", selection: $settings.textSpeed) {
                        Text("Instant").tag(0)
                        Text("Fast").tag(1)
                        Text("Medium").tag(3)
                        Text("Slow").tag(5)
                    }
                    Toggle("Hold A or B to speed it up", isOn: $settings.holdToSpeedText)
                }
                Section("Moving") {
                    Picker("Walking speed", selection: $settings.walkSpeed) {
                        Text("Original").tag(1)
                        Text("2×").tag(2)
                        Text("3×").tag(3)
                        Text("4×").tag(4)
                    }
                    Toggle("Hold B to run", isOn: $settings.runWithB)
                    if settings.runWithB {
                        Picker("Running speed", selection: $settings.runSpeed) {
                            Text("2×").tag(2)
                            Text("3×").tag(3)
                            Text("4×").tag(4)
                        }
                    }
                }
                Section("Look") {
                    Picker("Colours", selection: $settings.colors) {
                        Text("Super Game Boy").tag(0)
                        Text("Classic green").tag(1)
                        Text("Grey").tag(2)
                    }
                    Picker("Screen", selection: $settings.fillScreen) {
                        Text("Fill the screen").tag(true)
                        Text("Original size").tag(false)
                    }
                    Picker("Pixels", selection: $settings.smoothPixels) {
                        Text("Sharp").tag(false)
                        Text("Smooth").tag(true)
                    }
                    Toggle("Fade through doors", isOn: $settings.fades)
                }
                Section("Controls") {
                    VStack(alignment: .leading) {
                        Text("Button size")
                        Slider(value: $settings.controlsSize, in: 0.7...1.4)
                    }
                    VStack(alignment: .leading) {
                        Text("See-through")
                        Slider(value: $settings.controlsOpacity, in: 0.2...1.0)
                    }
                    Toggle("Vibrate on press", isOn: $settings.haptics)
                    Toggle("Keep touch buttons with a controller", isOn: $settings.showControlsWithController)
                }
                Section {
                    LabeledContent("Version", value: Bundle.main.versionLine)
                    Text("Thanks to the pret community, whose work documents how the original games run.")
                        .font(.footnote)
                        .foregroundStyle(.secondary)
                }
            }
            .navigationTitle("Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }
}

extension Bundle {
    var versionLine: String {
        let v = infoDictionary?["CFBundleShortVersionString"] as? String ?? "?"
        let b = infoDictionary?["CFBundleVersion"] as? String ?? "?"
        return "\(v) (build \(b))"
    }
}
