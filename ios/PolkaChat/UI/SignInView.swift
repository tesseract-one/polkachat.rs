//
//  SignInView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 02/03/2023.
//

import SwiftUI

struct SignInView: View {
    let signIn: () -> Void
    
    init(signIn: @escaping () -> Void) {
        self.signIn = signIn
    }
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("To start sending messages, please, sign in")
            
            Button(action: signIn) {
                Image(systemName: "person.text.rectangle") //rectangle.and.pencil.and.ellipsis
                Text("Sign-in with Tesseract")
            }.padding(.top)
        }
    }
}

struct SignInView_Previews: PreviewProvider {
    static var previews: some View {
        SignInView(signIn: {})
    }
}
