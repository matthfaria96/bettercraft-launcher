// BetterCraft Updater: in-app updates for BetterCraft Launcher through Sparkle 2.
//
// The launcher is Rust (egui) with no unsafe code, so Sparkle runs in this small helper inside the
// launcher's bundle (Contents/Helpers/BetterCraft Updater.app). It updates the launcher's bundle, not
// itself: the feed (SUFeedURL), the EdDSA key (SUPublicEDKey) and the daily check are read from the
// launcher's Info.plist, and Sparkle keeps its state in the launcher's defaults.
//
//   BetterCraft Updater --parent <pid>            started with the launcher: daily scheduled checks
//                                                 while the launcher runs, quits after it
//   BetterCraft Updater --parent <pid> --check    "Check for Launcher Updates…": always shows a result
//
// A second copy hands --check to the one already running and quits. The helper has no Dock icon,
// except while Sparkle shows an update window (as in Sirena), so that window is easy to find.

import AppKit
import Sparkle

private let checkNotification = Notification.Name("app.witchmymac.bettercraft-updater.check")

final class UpdaterDelegate: NSObject, NSApplicationDelegate, SPUUpdaterDelegate, SPUStandardUserDriverDelegate {
    private let host: Bundle
    private let parent: pid_t?
    private let checkAtStart: Bool
    private var updater: SPUUpdater?
    private var showingUpdate = false
    private var parentTimer: Timer?

    init(host: Bundle, parent: pid_t?, checkAtStart: Bool) {
        self.host = host
        self.parent = parent
        self.checkAtStart = checkAtStart
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        let driver = SPUStandardUserDriver(hostBundle: host, delegate: self)
        let updater = SPUUpdater(hostBundle: host, applicationBundle: host, userDriver: driver, delegate: self)
        do {
            try updater.start()
        } catch {
            NSLog("BetterCraft Updater: Sparkle didn't start: %@", error.localizedDescription)
            NSApp.terminate(nil)
            return
        }
        self.updater = updater
        DistributedNotificationCenter.default().addObserver(self, selector: #selector(checkRequested), name: checkNotification, object: nil)
        // Quit with the launcher (a timer as well: a launcher started from Terminal may not be an app).
        NSWorkspace.shared.notificationCenter.addObserver(self, selector: #selector(appTerminated(_:)), name: NSWorkspace.didTerminateApplicationNotification, object: nil)
        parentTimer = Timer.scheduledTimer(withTimeInterval: 30, repeats: true) { [weak self] _ in self?.quitIfParentGone() }
        if checkAtStart { checkRequested() }
    }

    @objc private func checkRequested() {
        guard let updater, updater.canCheckForUpdates else { return }
        showDockIcon()
        NSApp.activate(ignoringOtherApps: true)
        updater.checkForUpdates()
    }

    @objc private func appTerminated(_ note: Notification) {
        let app = note.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication
        if let parent, app?.processIdentifier == parent { quitIfParentGone() }
    }

    private func quitIfParentGone() {
        guard let parent, kill(parent, 0) != 0, !showingUpdate else { return }
        NSApp.terminate(nil)
    }

    // MARK: Dock icon while an update window is up

    private func showDockIcon() {
        showingUpdate = true
        if NSApp.activationPolicy() != .regular { NSApp.setActivationPolicy(.regular) }
    }

    private func hideDockIcon() {
        showingUpdate = false
        NSApp.setActivationPolicy(.accessory)
        quitIfParentGone()
    }

    var supportsGentleScheduledUpdateReminders: Bool { true }

    func standardUserDriverShouldHandleShowingScheduledUpdate(_ update: SUAppcastItem, andInImmediateFocus immediateFocus: Bool) -> Bool {
        true
    }

    func standardUserDriverWillHandleShowingUpdate(_ handleShowingUpdate: Bool, forUpdate update: SUAppcastItem, state: SPUUserUpdateState) {
        showDockIcon()
        if !state.userInitiated, !NSApp.isActive {
            NSApp.requestUserAttention(.informationalRequest)
        }
    }

    func standardUserDriverWillFinishUpdateSession() {
        hideDockIcon()
    }

    // MARK: Installing over the running launcher

    /// Sparkle replaces the launcher's bundle once this helper quits, then opens the new version.
    /// The launcher must not be running by then, or macOS would only bring the old copy forward.
    func updaterWillRelaunchApplication(_ updater: SPUUpdater) {
        let hostURL = host.bundleURL.standardizedFileURL
        let running = NSWorkspace.shared.runningApplications.filter {
            $0.processIdentifier != getpid() && $0.bundleURL?.standardizedFileURL == hostURL
        }
        running.forEach { $0.terminate() }
        let deadline = Date().addingTimeInterval(10)
        while running.contains(where: { !$0.isTerminated }), Date() < deadline {
            RunLoop.current.run(until: Date().addingTimeInterval(0.2))
        }
        running.filter { !$0.isTerminated }.forEach { $0.forceTerminate() }
    }
}

/// The launcher's bundle: this helper lives at <Launcher>.app/Contents/Helpers/BetterCraft Updater.app.
private func launcherBundle() -> Bundle? {
    let url = Bundle.main.bundleURL.deletingLastPathComponent().deletingLastPathComponent().deletingLastPathComponent()
    guard url.pathExtension == "app" else { return nil }
    return Bundle(url: url)
}

let arguments = CommandLine.arguments
let wantsCheck = arguments.contains("--check")
let parent = arguments.firstIndex(of: "--parent").flatMap { arguments.indices.contains($0 + 1) ? pid_t(arguments[$0 + 1]) : nil }

guard let host = launcherBundle() else {
    NSLog("BetterCraft Updater: not inside BetterCraft Launcher.app")
    exit(1)
}
// Test copies (another bundle id) never check, unless BETTERCRAFT_UPDATES=1.
guard host.bundleIdentifier == "ai.storyteller.artcraft-launcher" || ProcessInfo.processInfo.environment["BETTERCRAFT_UPDATES"] != nil else {
    exit(0)
}
let others = NSRunningApplication.runningApplications(withBundleIdentifier: Bundle.main.bundleIdentifier ?? "")
    .filter { $0.processIdentifier != getpid() }
if !others.isEmpty {
    if wantsCheck {
        DistributedNotificationCenter.default().postNotificationName(checkNotification, object: nil, userInfo: nil, deliverImmediately: true)
    }
    exit(0)
}

let app = NSApplication.shared
let delegate = UpdaterDelegate(host: host, parent: parent, checkAtStart: wantsCheck)
app.delegate = delegate
app.setActivationPolicy(.accessory)
app.run()
