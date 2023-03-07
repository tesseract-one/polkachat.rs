//
//  ViewModel.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import Foundation

class ViewModel: ObservableObject {
    let core: Core
    let error: ErrorModel
    
    @Published var account: String?
    @Published var messages: Array<Message>
    
    init(core: Core, error: ErrorModel) {
        self.core = core
        self.error = error
        self.messages = []
        
        Task { @MainActor in
            do {
                let messages = try await core.messages(from: 0).map { message in
                    Message.newCommited(text: message)
                }
                self.messages.append(contentsOf: messages)
            } catch {
                await presentError(error: error)
            }
        }
    }
    
    func login() {
        Task { @MainActor in
            do {
                account = try await self.core.account()
            } catch {
                await presentError(error: error)
            }
        }
        
        //account = "thisistheaccountplaceholder2"
    }
    
    func sendMessage(message: String) {
        Task { @MainActor in
            let text = message
            let message = Message.newSubmitted(text: message)
            
            do {
                messages.append(message)
                try await core.send(message: text)
                
                let index = messages.lastIndex(of: message)!
                messages[index] = message.intoCommited()
            } catch {
                messages.removeAll { $0 == message }
                
                await presentError(error: error)
            }
        }
    }
    
    func presentError(error: Swift.Error) async {
        await self.error.presentError(error: error)
    }
}
