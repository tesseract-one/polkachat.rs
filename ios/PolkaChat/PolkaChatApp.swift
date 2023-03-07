//
//  PolkaChatApp.swift
//  PolkaChat
//
//  Created by Daniel Leping on 15/02/2023.
//

import SwiftUI
import CPolkaChat

@main
struct PolkaChatApp: App {
    private let model: ViewModel
    private let error: ErrorModel
    
    init() {
        let errorModel = ErrorModel()
        let ui = UI(errorModel: errorModel)
        
        self.model = ViewModel(core: try! Core(ui: ui))
        self.error = errorModel
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView(model: model, error: error)
        }
    }
}
