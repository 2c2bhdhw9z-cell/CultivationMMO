import Foundation

/// The player's own cartridge files, kept privately inside the app, one per game, plus the
/// spot each game was left at.
final class Cartridges: ObservableObject {
    static let shared = Cartridges()

    @Published private(set) var installed: Set<Game> = []

    private let root: URL = {
        let base = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0]
        return base.appendingPathComponent("Retold", isDirectory: true)
    }()

    private var cartFolder: URL { root.appendingPathComponent("Cartridges", isDirectory: true) }
    private var saveFolder: URL { root.appendingPathComponent("Resume", isDirectory: true) }

    init() {
        for dir in [cartFolder, saveFolder] {
            try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        }
        refresh()
    }

    private func cartURL(_ g: Game) -> URL { cartFolder.appendingPathComponent("\(g.rawValue).cart") }
    private func resumeURL(_ g: Game) -> URL { saveFolder.appendingPathComponent("\(g.rawValue).resume") }

    func refresh() {
        installed = Set(Game.allCases.filter { FileManager.default.fileExists(atPath: cartURL($0).path) })
    }

    enum AddResult {
        case added(Game)
        case notACartridge
        case failed
    }

    /// Reads a file the player picked and keeps the cartridge in it.
    func add(from url: URL) -> AddResult {
        let scoped = url.startAccessingSecurityScopedResource()
        defer { if scoped { url.stopAccessingSecurityScopedResource() } }
        guard let data = try? Data(contentsOf: url) else { return .failed }
        guard let (game, cart) = findCartridge(in: data) else { return .notACartridge }
        do {
            try cart.write(to: cartURL(game), options: .atomic)
        } catch {
            return .failed
        }
        refresh()
        return .added(game)
    }

    func cartridge(_ g: Game) -> Data? { try? Data(contentsOf: cartURL(g)) }

    func remove(_ g: Game) {
        try? FileManager.default.removeItem(at: cartURL(g))
        try? FileManager.default.removeItem(at: resumeURL(g))
        refresh()
    }

    func resume(_ g: Game) -> Data? { try? Data(contentsOf: resumeURL(g)) }

    func saveResume(_ g: Game, _ data: Data) {
        try? data.write(to: resumeURL(g), options: .atomic)
    }

    func forgetResume(_ g: Game) {
        try? FileManager.default.removeItem(at: resumeURL(g))
    }
}
