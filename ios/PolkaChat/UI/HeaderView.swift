//
//  HeaderView.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import SwiftUI

struct HeaderView: View {
    var body: some View {
        ZStack(alignment: .leading) {
            Color(red: 0x4A/0xFF,
                  green: 0x93/0xFF,
                  blue: 0xE2/0xFF)
            .edgesIgnoringSafeArea(.top)
            VStack(alignment: .leading) {
                Text("Tesseract")
                    .font(.system(size: 48))
                    .foregroundColor(Color.white)
                    .padding(.bottom, 1)
                Text("Polkadot Demo dApp")
                    .font(.system(size: 32))
                    .foregroundColor(Color.white)
                    .padding(.bottom, 1)
                Text("This dApp is a simple chat room made with smart contracts on the Polkadot network to demonstrate the Tesseract dApp/Wallet integration.")
                    .padding(.bottom)
                    .foregroundColor(Color.white)
                    .lineLimit(nil)
                    .fixedSize(horizontal: false, vertical: true)
            }.padding(.horizontal)
        }.aspectRatio(contentMode: .fit)
    }
}

struct HeaderView_Previews: PreviewProvider {
    static var previews: some View {
        HeaderView()
    }
}
