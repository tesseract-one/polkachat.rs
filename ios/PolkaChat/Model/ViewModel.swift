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
        self.messages = []
        
        Task { @MainActor in
            do {
                let messages = try await core.messages(from: 0).map { message in
                    Message.newCommited(text: message)
                }
                self.messages.append(contentsOf: messages)
            } catch {
                print("Error: \(error)")
            }
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
