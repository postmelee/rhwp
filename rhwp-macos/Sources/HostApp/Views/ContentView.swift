import SwiftUI

struct ContentView: View {
    @ObservedObject var store: DocumentViewerStore

    var body: some View {
        DocumentViewerView(store: store)
        .toolbar {
            ToolbarItemGroup {
                Button {
                    store.openDocument()
                } label: {
                    Label("Open Document", systemImage: "folder")
                }

                Button {
                    store.loadSample()
                } label: {
                    Label("Open Sample", systemImage: "doc")
                }
            }

            ToolbarItemGroup {
                Button {
                    store.zoomOut()
                } label: {
                    Label("Zoom Out", systemImage: "minus.magnifyingglass")
                }
                .disabled(!store.hasDocument)

                Slider(
                    value: $store.zoomScale,
                    in: store.minimumZoomScale...store.maximumZoomScale
                )
                .frame(width: 130)
                .disabled(!store.hasDocument)

                Button {
                    store.zoomIn()
                } label: {
                    Label("Zoom In", systemImage: "plus.magnifyingglass")
                }
                .disabled(!store.hasDocument)

                Button {
                    store.resetZoom()
                } label: {
                    Label("Actual Size", systemImage: "arrow.up.left.and.down.right.magnifyingglass")
                }
                .disabled(!store.hasDocument)
            }
        }
    }
}
