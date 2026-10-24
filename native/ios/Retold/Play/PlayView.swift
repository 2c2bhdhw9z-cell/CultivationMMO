import SwiftUI

/// Playing a game: the picture, the controls, and a small menu button.
struct PlayView: View {
    let game: Game
    @Environment(\.dismiss) private var dismiss
    @State private var surface = PlaySurface()
    @State private var failed: String?
    @State private var showMenu = false
    @State private var showSettings = false
    @State private var leaving = false

    var body: some View {
        ZStack(alignment: .topTrailing) {
            SurfaceHost(surface: surface)
                .ignoresSafeArea()
            if let failed {
                VStack(spacing: 16) {
                    Text(failed)
                        .multilineTextAlignment(.center)
                        .foregroundStyle(.white)
                    Button("Back to games") { dismiss() }
                        .buttonStyle(.borderedProminent)
                }
                .padding(32)
                .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else {
                Button {
                    surface.running = false
                    surface.saveNow()
                    showMenu = true
                } label: {
                    Image(systemName: "pause.fill")
                        .font(.system(size: 15, weight: .bold))
                        .foregroundStyle(.white.opacity(0.9))
                        .frame(width: 40, height: 40)
                        .background(.ultraThinMaterial, in: Circle())
                }
                .padding(.trailing, 14)
                .padding(.top, 4)
            }
        }
        .statusBarHidden()
        .persistentSystemOverlays(.hidden)
        .onAppear(perform: start)
        .onDisappear {
            surface.running = false
            surface.saveNow()
        }
        .confirmationDialog("Paused", isPresented: $showMenu, titleVisibility: .visible) {
            Button("Keep playing") { surface.running = true }
            Button("Settings") { showSettings = true }
            Button("Back to games") {
                leaving = true
                surface.saveNow()
                dismiss()
            }
        }
        .onChange(of: showMenu) { open in
            // Closing the menu by tapping outside it carries on playing.
            if !open && !showSettings && !leaving && failed == nil { surface.running = true }
        }
        .sheet(isPresented: $showSettings, onDismiss: {
            surface.applySettings()
            surface.running = true
        }) {
            SettingsView()
        }
    }

    private func start() {
        guard surface.session == nil else { return }
        guard let cart = Cartridges.shared.cartridge(game), let session = Session(game: game, cartridge: cart) else {
            failed = "Couldn't start \(game.name). Try removing the cartridge and adding it again."
            return
        }
        if let resume = Cartridges.shared.resume(game) {
            session.load(resume)
        }
        surface.game = game
        surface.session = session
        surface.applySettings()
        surface.running = true
    }
}

private struct SurfaceHost: UIViewRepresentable {
    let surface: PlaySurface
    func makeUIView(context: Context) -> PlaySurface { surface }
    func updateUIView(_ view: PlaySurface, context: Context) {}
}
