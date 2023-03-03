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
            HeaderView()
            VStack {
                ScrollView {
                    MessagesView(messages: ["one", "two", "three"])
                }
                
                if let account = account {
                    UserControlsView(account: account) { message in
                        print("Message to send: \(message)")
                    }
                    .padding()
                    .transition(.opacity.animation(.easeInOut) )
                } else {
                    HStack {
                        SignInView {
                            account = "thisistheaccountmock"
                        }
                        
                        Spacer()
                    }
                    .padding()
                    .transition(.opacity.animation(.easeInOut))
                }
            }.padding()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
