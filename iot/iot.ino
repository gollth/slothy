#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <ArduinoJson.h>
#include <WiFiClient.h>
#include "credentials.h"

#define D0       16
#define D1        5
#define D2        4
#define D3        0
#define D4        2
#define D5       14
#define D6       12
#define D7       13
#define D8       15

#define MUX_A   D6
#define MUX_B   D5
#define MUX_C   D7

#define DEEP_SLEEP           D0
#define DEEP_SLEEP_PERIOD_S  30

#define URL_BASE   "http://homeassistant.local:8123/api/states/sensor."

#define ANA_SRC_VOLTAGE    6
#define ANA_SRC_HUMID_0    5
#define ANA_SRC_HUMID_1    3
#define ANA_SRC_HUMID_2    1
#define ANA_SRC_HUMID_3    7
#define ANA_SRC_HUMID_4    2
#define ANA_SRC_HUMID_5    0

#define ANA_VOLTAGE_REF    3.3     // Reference voltage corresponding to 1023 of ADC [V]

#define HUMIDITY_ADC_DRY   550     // ADC value when reading complete dry soil [0..1023]
#define HUMIDITY_ADC_WET   300     // ADC value when reading complete wet soid [0..1023]
#define VOLTAGE_DIVIDER_R1 97.7e3  // Low-side resistor of voltage divider [Ohm]
#define VOLTAGE_DIVIDER_R2 47.3e3  // High-side resistor of voltage divider [Ohm]

char ANA_SRC_HUMIDS[6] = {
  ANA_SRC_HUMID_0,
  ANA_SRC_HUMID_1,
  ANA_SRC_HUMID_2,
  ANA_SRC_HUMID_3,
  ANA_SRC_HUMID_4,
  ANA_SRC_HUMID_5
};

String SENSOR_NAMES[6] = {
  "Lauch/Minze",
  "Pflücksalat",
  "Zwiebeln (links)",
  "Zwiebeln (rechts)",
  "Knoblauch",
  "Schnittlauch/Petersilie"
};

#define BATTERY_NAME  "Batterie (Ost)"

void setup() {
  Serial.begin(115200);
  pinMode(MUX_A, OUTPUT);
  pinMode(MUX_B, OUTPUT);
  pinMode(MUX_C, OUTPUT);
  pinMode(A0, INPUT);
  
  uint64_t id = ESP.getChipId();
  Serial.printf("ESP8266 Chip ID: %llu\n", id);

  connect_to_wifi();
  WiFiClient wifi;

  float voltage = readVoltage();
  Serial.print(">> Voltage: "); Serial.print(voltage); Serial.println(" V");
  send_to_backend(wifi, id, 'v', BATTERY_NAME, String(voltage, 3), "V");
  for (int i = 0; i < 6; i++) {
    int h = readAnalogFrom(ANA_SRC_HUMIDS[i]);
    float humidity = calculateHumitiy(h);
    Serial.print(">> Humidity #: "); Serial.print(i); Serial.print(": ");
    Serial.print(humidity, 0); Serial.print(" %");
    Serial.print(" ("); Serial.print(h); Serial.println(")");

    if (not isnan(humidity)) {
      send_to_backend(wifi, id, i + '1', SENSOR_NAMES[i].c_str(), String(humidity, 0), "%");
    }
  }

  deep_sleep();
}

void connect_to_wifi() {
  WiFi.begin(WIFI_SSID, WIFI_PASS);
  Serial.println("");
  Serial.println("Connecting");
  while(WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.print("Connected to WiFi network with IP Address: ");
  Serial.println(WiFi.localIP());
}

void send_to_backend(WiFiClient& wifi, uint64_t id, char sensor, const char* title, String value, String unit) {
  String url = String(URL_BASE) + String("slothy_") + String(id, HEX) + "_" + sensor;
  
  HTTPClient http;
  http.begin(wifi, url);
  http.addHeader("Content-Type", "application/json");
  http.addHeader("Authorization", String("Bearer ") + String(LONG_LIVED_ACCESS_TOKEN));

  // Refer to https://arduinojson.org/v5/assistant/ for size calculation
  StaticJsonDocument<170> json;
  json["state"] = value;

  JsonObject attributes = json.createNestedObject("attributes");
  attributes["unit_of_measurement"] = unit;
  attributes["friendly_name"] = title;
  
  char buffer[1024];
  serializeJson(json, buffer);
  Serial.print("POST: "); Serial.println(url);
  Serial.print ("    >> "); Serial.println(buffer);

  int code = http.POST(buffer);
  Serial.print("    << "); Serial.println(code);
  Serial.print("    << "); Serial.println(http.getString());
  
  http.end();
}


int readAnalogFrom(char channel) {
  digitalWrite(MUX_A, channel & (1 << 0));
  digitalWrite(MUX_B, channel & (1 << 1));
  digitalWrite(MUX_C, channel & (1 << 2));
  return analogRead(A0);
}

float calculateHumitiy(int x) {
  if(x > HUMIDITY_ADC_DRY + 25 || x < HUMIDITY_ADC_WET - 25) {
    return NAN;
  }

  return 100.0 - 100.0 * ((float)x - HUMIDITY_ADC_WET) / (HUMIDITY_ADC_DRY - HUMIDITY_ADC_WET);
}
float readVoltage() {
  float x = readAnalogFrom(ANA_SRC_VOLTAGE);
  return (x * ANA_VOLTAGE_REF * (VOLTAGE_DIVIDER_R1 + VOLTAGE_DIVIDER_R2)) / (VOLTAGE_DIVIDER_R1 * 1023);
}


void deep_sleep() {
  Serial.println("Entering deep sleep...");
  ESP.deepSleep(DEEP_SLEEP_PERIOD_S * 1e6);
  yield();
}

void loop() {}
