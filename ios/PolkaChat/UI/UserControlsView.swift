//
//  UserControlsView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 02/03/2023.
//

import SwiftUI

struct UserControlsView: View {
    @State var text: String
    
    let account: String
    let sendMessage: (String) -> Void
    
    init(account: String, sendMessage: @escaping (String) -> Void) {
        self.text = ""
        
        self.account = account
        self.sendMessage = sendMessage
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Account ID: \(account)")
            HStack {
                TextField("Message", text: $text)
                
                Button(action: {
                    let text = text
                    self.text = ""
                    sendMessage(text)
                }) {
                    Image(systemName: "paperplane.fill").rotationEffect(.degrees(45))
                }.disabled(text == "")
            }
        }
    }
}

struct UserControlsView_Previews: PreviewProvider {
    static var previews: some View {
        UserControlsView(account: "thisismypreviewaccount") {message in
            print("Message sent: \(message)")
        }
    }
}
