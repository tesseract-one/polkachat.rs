package one.tesseract.polkachat

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import one.tesseract.polkachat.ui.main.MainFragment

class MainActivity : AppCompatActivity() {
    companion object {
        init {
            System.loadLibrary("polkachat")
        }
    }

    external fun test()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        if (savedInstanceState == null) {
            supportFragmentManager.beginTransaction()
                .replace(R.id.container, MainFragment.newInstance())
                .commitNow()
        }

        test()
    }
}