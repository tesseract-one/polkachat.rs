//
//  ViewModel.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import Foundation

class ViewModel: ObservableObject {
    let core: Core
    
    @Published var account: String?
    @Published var messages: Array<Message>
    
    init(core: Core) {
        self.core = core
        self.messages = Array(0...1000).map { num in
            Message.newCommited(text: "message: \(num)")
        }
    }
    
    func login() {
        Task {
            do {
                account = try await self.core.account()
            } catch {
                //signed = "Error: \(error)"
                print("Error: \(error)")
            }
        }
        
        //account = "thisistheaccountplaceholder2"
    }
    
    func sendMessage(message: String) {
        messages.append(Message.newSubmitted(text: message))
    }
    
    func prosentError(error: String) {
        print("Error: \(error)")
    }
}
