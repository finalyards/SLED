## Board support

>The library is intended to be a development aid: a way to signal internal state to the human world. For this reason, the focus is solely on devkits.

Common ESP32-C6 devkits and their configurations:

||LED type|GPIO pin|color mode|comments|
|---|---|---|---|---|
|[ESP32-C6-DevKitM-1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)|WS2812|8|GRB|Espressif official; mini module|
|[ESP32-C6-DevKitC-1 v1.2](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html)|WS2812|8|GRB|Espressif official; "C" module|
|[Adafruit ESP32-C6 Feather](https://www.adafruit.com/product/5933?srsltid=AfmBOoqnU8XMK_-68vxFrnrW5sF4hgaiZpy3mGYIpgWXwHxSADMy3_sX)|NeoPixel (WS2812)|9|GRB|Note: also needs GPIO 20 to be pulled up.|
|[SparkFun Thing Plus - ESP32-C6](https://www.sparkfun.com/sparkfun-thing-plus-esp32-c6.html?srsltid=AfmBOoptJP-n3Ws70TUbejpllWgYaqRPJn_1wU3XRO-D7xJU20EuUEKU)|WS2812|23|GRB||
|[ESP32-C6 SuperMini](https://piikauppa.fi/kauppa/mikrokontrollerit/esp32/esp32-c6-supermini/?srsltid=AfmBOoqj-9soDvqdnl63FXHYSZNrAepwxQM3rhoJPvkqLRhPCk7n3ObJ)|WS2812|8|GRB|Very small, no module!|
|[Waveshare ESP32-C6 0.85inch LCD Display](https://openelab.io/fi/products/waveshare-esp32-c6-85inch-lcd?srsltid=AfmBOoo_0gsJtgn-n6WmdvoXWbyErl_ex7r8hUl23FO8yrD_PuG4BtZQ)|WS2812B x 8|14|RGB or GRB|8 "surround" LEDs!!|

>Note! Technical specs via `google.ai` - let me know if there are mistakes!!!

Initially, only the Espressif boards are supported, but the intention is to enlarge `feature` support, over time.

