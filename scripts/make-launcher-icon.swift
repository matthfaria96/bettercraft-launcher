// BetterCraft: turns assets/bettercraft/launcher-icon-source.png (full-bleed 1024 px art) into a
// macOS-shaped icon: an 824 px rounded square centred on a transparent 1024 px canvas.
// Usage: swift scripts/make-launcher-icon.swift <source.png> <out.png>
import AppKit

let args = CommandLine.arguments
guard args.count == 3, let src = NSImage(contentsOfFile: args[1]) else {
    FileHandle.standardError.write("usage: make-launcher-icon.swift <source.png> <out.png>\n".data(using: .utf8)!)
    exit(1)
}
let canvas = 1024, body: CGFloat = 824, radius: CGFloat = 185
let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: canvas, pixelsHigh: canvas, bitsPerSample: 8,
                           samplesPerPixel: 4, hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
                           bytesPerRow: 0, bitsPerPixel: 0)!
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
NSGraphicsContext.current?.imageInterpolation = .high
let inset = (CGFloat(canvas) - body) / 2
let rect = NSRect(x: inset, y: inset, width: body, height: body)
NSBezierPath(roundedRect: rect, xRadius: radius, yRadius: radius).addClip()
src.draw(in: rect)
NSGraphicsContext.restoreGraphicsState()
guard let png = rep.representation(using: .png, properties: [:]) else { exit(1) }
try png.write(to: URL(fileURLWithPath: args[2]))
