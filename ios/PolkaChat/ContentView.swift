//
//  ContentView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 15/02/2023.
//

import SwiftUI
import CPolkaChat

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
            Button(action: {
                polkachat_app_test()
            }, label: {
                Text("Test")
            })
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
