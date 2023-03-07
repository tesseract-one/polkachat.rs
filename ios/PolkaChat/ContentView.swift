//
//  ContentView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 15/02/2023.
//

import SwiftUI
import CPolkaChat

struct ContentView: View {
    @StateObject private var model: ViewModel
    @StateObject private var error: ErrorModel
    
    init(model: ViewModel, error: ErrorModel) {
        _model = StateObject(wrappedValue: model)
        _error = StateObject(wrappedValue: error)
    }
    
    var body: some View {
        VStack(spacing: 0) {
            HeaderView()
            VStack {
                MessagesView(messages: model.messages)
                
                if let account = model.account {
                    UserControlsView(account: account, sendMessage: model.sendMessage)
                    .padding()
                    .transition(.opacity.animation(.easeInOut) )
                } else {
                    HStack {
                        SignInView(signIn: model.login)
                        Spacer()
                    }
                    .padding()
                    .transition(.opacity.animation(.easeInOut))
                }
            }
            .padding(.horizontal)
            .padding(.bottom)
        }.alert(item: $error.error) { error in
            Alert(title: Text("Error"), message: Text(error.message))
        }
    }
}

//struct ContentView_Previews: PreviewProvider {
//    static var previews: some View {
//        ContentView(model: ViewModel())
//    }
//}
