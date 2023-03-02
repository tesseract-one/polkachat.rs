//
//  ContentView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 15/02/2023.
//

import SwiftUI
import CPolkaChat

struct ContentView: View {
    @State var account: String?
    
    var body: some View {
        VStack {
            Text("Polkadot Demo dApp")
            Text("This dApp is a simple chat room made with smart contracts on the Polkadot network.")
            
            MessagesView(messages: ["one", "two", "three"])
            
            if let account = account {
                UserControlsView(account: account) { message in
                    print("Message to send: \(message)")
                }
            } else {
                SignInView {
                    account = "thisistheaccountmock"
                }
            }
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
