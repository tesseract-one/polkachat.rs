//
//  ViewModel.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import Foundation

class ViewModel: ObservableObject {
    @Published var account: String?
    @Published var messages: Array<String>
    
    init() {
        self.messages = Array(0...1000).map { num in
            "message: \(num)"
        }
    }
    
    func login() {
        account = "thisistheaccountplaceholder2"
    }
    
    func sendMessage(message: String) {
        messages.append(message)
    }
    
    func prosentError(error: String) {
        print("Error: \(error)")
    }
}
