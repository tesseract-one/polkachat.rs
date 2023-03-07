//
//  AlertProvider.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation

class ErrorModel: ObservableObject {
    public struct Error: Identifiable {
        let message: String
        public var id: String { message }
    }
    
    @Published var error: Error? = nil
    
    @MainActor
    func presentError(message: String) {
        self.error = Error(message: message)
    }
}