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
    @State var messages: Array<String>
    
    init() {
        self.messages = Array(0...1000).map { num in
            "message: \(num)"
        }
    }
    
    var body: some View {
        VStack(spacing: 0) {
            HeaderView()
            VStack {
                MessagesView(messages: messages)
                
                if let account = account {
                    UserControlsView(account: account) { message in
                        print("Message to send: \(message)")
                    }
                    .padding()
                    .transition(.opacity.animation(.easeInOut) )
                } else {
                    HStack {
                        SignInView {
                            //account = "thisistheaccountmock"
                            let rand = Int.random(in: 1001...10000)
                            messages.append("new message\(rand)")
                        }
                        
                        Spacer()
                    }
                    .padding()
                    .transition(.opacity.animation(.easeInOut))
                }
            }
            .padding(.horizontal)
            .padding(.bottom)
            //.background(.red)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
