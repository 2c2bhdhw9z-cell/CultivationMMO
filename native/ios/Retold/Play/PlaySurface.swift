import GameController
import UIKit

/// The play screen in UIKit: the game picture and the touch controls, laid out for portrait or
/// landscape, and the 60-frames-a-second loop that runs the engine.
final class PlaySurface: UIView {
    let screen = UIView()
    let controls = ControlsView()
    var session: Session? {
        // A new game needs the view size pushed to it on the next layout.
        didSet { viewTiles = (0, 0) }
    }
    var game: Game?

    /// Runs the game when true.
    var running = false {
        didSet {
            link?.isPaused = !running
            if !running { controls.releaseAll() }
            last = 0
        }
    }

    private var link: CADisplayLink?
    private var last: CFTimeInterval = 0
    private var carry: CFTimeInterval = 0
    private var touchPad: Pad = []
    private var lastSave: CFTimeInterval = 0
    private var viewTiles = (w: 20, h: 18)

    override init(frame: CGRect) {
        super.init(frame: frame)
        backgroundColor = .black
        screen.layer.magnificationFilter = .nearest
        screen.layer.contentsGravity = .resizeAspect
        screen.isUserInteractionEnabled = false
        addSubview(screen)
        addSubview(controls)
        controls.onChange = { [weak self] pad in self?.touchPad = pad }
        let center = NotificationCenter.default
        center.addObserver(self, selector: #selector(controllersChanged), name: .GCControllerDidConnect, object: nil)
        center.addObserver(self, selector: #selector(controllersChanged), name: .GCControllerDidDisconnect, object: nil)
        center.addObserver(self, selector: #selector(saveNow), name: UIApplication.willResignActiveNotification, object: nil)
    }

    required init?(coder: NSCoder) { fatalError() }

    deinit {
        link?.invalidate()
        NotificationCenter.default.removeObserver(self)
    }

    override func didMoveToWindow() {
        super.didMoveToWindow()
        if window != nil, link == nil {
            let l = CADisplayLink(target: WeakTarget(self), selector: #selector(WeakTarget.tick(_:)))
            l.preferredFrameRateRange = CAFrameRateRange(minimum: 60, maximum: 60, preferred: 60)
            l.add(to: .main, forMode: .common)
            l.isPaused = !running
            link = l
        } else if window == nil {
            link?.invalidate()
            link = nil
        }
    }

    @objc private func controllersChanged() { applySettings() }

    /// Picks up changed settings: look, screen size, controls.
    func applySettings() {
        let s = Settings.shared
        if let session { s.apply(to: session) }
        screen.layer.magnificationFilter = s.smoothPixels ? .linear : .nearest
        controls.sizeScale = CGFloat(s.controlsSize)
        controls.alpha = CGFloat(s.controlsOpacity)
        controls.haptics = s.haptics
        let hasController = !GCController.controllers().isEmpty
        controls.isHidden = hasController && !s.showControlsWithController
        setNeedsLayout()
        layoutIfNeeded()
        draw()
    }

    override func layoutSubviews() {
        super.layoutSubviews()
        let b = bounds
        let inset = safeAreaInsets
        let landscape = b.width > b.height
        let area: CGRect
        if landscape {
            area = CGRect(x: 0, y: inset.top, width: b.width, height: b.height - inset.top - inset.bottom)
            controls.controlArea = b
        } else {
            let controlsHeight = controls.isHidden ? 0 : max(270, b.height * 0.40)
            let top = inset.top + 52
            area = CGRect(x: 0, y: top, width: b.width, height: max(144, b.height - controlsHeight - top))
            controls.controlArea = CGRect(x: 0, y: area.maxY, width: b.width, height: b.height - area.maxY)
        }
        controls.landscape = landscape
        controls.frame = b
        screen.frame = area
        // How much map fits: the original 20x18 tiles, or as many as fill this space.
        var w = 20
        var h = 18
        if Settings.shared.fillScreen, area.width > 0, area.height > 0 {
            let aspect = area.width / area.height
            if aspect >= 20.0 / 18.0 {
                w = Int((18 * aspect).rounded())
            } else {
                h = Int((20 / aspect).rounded())
            }
        }
        if w != viewTiles.w || h != viewTiles.h {
            viewTiles = (w, h)
            session?.setView(tilesWide: w, tilesHigh: h)
            draw()
        }
    }

    private func controllerPad() -> Pad {
        var pad: Pad = []
        for c in GCController.controllers() {
            guard let g = c.extendedGamepad else {
                if let m = c.microGamepad {
                    if m.dpad.up.isPressed { pad.insert(.up) }
                    if m.dpad.down.isPressed { pad.insert(.down) }
                    if m.dpad.left.isPressed { pad.insert(.left) }
                    if m.dpad.right.isPressed { pad.insert(.right) }
                    if m.buttonA.isPressed { pad.insert(.a) }
                    if m.buttonX.isPressed { pad.insert(.b) }
                }
                continue
            }
            let stick = g.leftThumbstick
            if g.dpad.up.isPressed || stick.yAxis.value > 0.5 { pad.insert(.up) }
            if g.dpad.down.isPressed || stick.yAxis.value < -0.5 { pad.insert(.down) }
            if g.dpad.left.isPressed || stick.xAxis.value < -0.5 { pad.insert(.left) }
            if g.dpad.right.isPressed || stick.xAxis.value > 0.5 { pad.insert(.right) }
            // Nintendo layout: the right-hand button is A.
            if g.buttonB.isPressed { pad.insert(.a) }
            if g.buttonA.isPressed { pad.insert(.b) }
            if g.buttonMenu.isPressed { pad.insert(.start) }
            if g.buttonOptions?.isPressed == true { pad.insert(.select) }
        }
        if let k = GCKeyboard.coalesced?.keyboardInput {
            func down(_ code: GCKeyCode) -> Bool { k.button(forKeyCode: code)?.isPressed == true }
            if down(.upArrow) || down(.keyW) { pad.insert(.up) }
            if down(.downArrow) || down(.keyS) { pad.insert(.down) }
            if down(.leftArrow) || down(.keyA) { pad.insert(.left) }
            if down(.rightArrow) || down(.keyD) { pad.insert(.right) }
            if down(.keyZ) || down(.spacebar) { pad.insert(.a) }
            if down(.keyX) || down(.deleteOrBackspace) { pad.insert(.b) }
            if down(.returnOrEnter) { pad.insert(.start) }
            if down(.tab) || down(.leftShift) || down(.rightShift) { pad.insert(.select) }
        }
        return pad
    }

    fileprivate func tick(_ link: CADisplayLink) {
        guard running, let session else { return }
        let now = link.timestamp
        if last == 0 {
            last = now
            carry = 1.0 / 60
        }
        carry += now - last
        last = now
        let frame = 1.0 / 60
        var steps = 0
        let buttons = touchPad.union(controllerPad()).rawValue
        while carry >= frame && steps < 4 {
            session.step(buttons: buttons)
            carry -= frame
            steps += 1
        }
        if carry > frame { carry = 0 }
        if steps > 0 { draw() }
        if now - lastSave > 10 {
            lastSave = now
            saveNow()
        }
    }

    func draw() {
        guard let image = session?.image() else { return }
        CATransaction.begin()
        CATransaction.setDisableActions(true)
        screen.layer.contents = image
        CATransaction.commit()
    }

    /// Remembers where you are, so the game opens there next time.
    @objc func saveNow() {
        guard let session, let game else { return }
        Cartridges.shared.saveResume(game, session.save())
    }
}

/// Keeps the display link from holding the view alive.
private final class WeakTarget: NSObject {
    weak var surface: PlaySurface?
    init(_ s: PlaySurface) { surface = s }
    @objc func tick(_ link: CADisplayLink) { surface?.tick(link) }
}
