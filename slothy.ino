#define D1   5
#define D2   4
#define D3   0

#define ANA_SRC_VOLTAGE    7
#define ANA_SRC_HUMID0     2
#define ANA_VOLTAGE_REF    3.3     // Reference voltage corresponding to 1023 of ADC [V]

#define HUMIDITY_ADC_DRY   800     // ADC value when reading complete dry soil [0..1023]
#define HUMIDITY_ADC_WET   200     // ADC value when reading complete wet soid [0..1023] 
#define VOLTAGE_DIVIDER_R1 92e3    // Low-side resistor of voltage divider [Ohm]
#define VOLTAGE_DIVIDER_R2 48e3    // High-side resistor of voltage divider [Ohm]

void setup() {
  Serial.begin(9600);
  pinMode(D1, OUTPUT);
  pinMode(D2, OUTPUT);
  pinMode(D3, OUTPUT);
  pinMode(A0, INPUT);
}


int readAnalogFrom(char channel) {
  digitalWrite(D1, channel & (1 << 0));
  digitalWrite(D2, channel & (1 << 1));
  digitalWrite(D3, channel & (1 << 2));
  return analogRead(A0);
}

float readHumitiy(char channel) {
  return ((float)readAnalogFrom(channel) - HUMIDITY_ADC_WET) / (HUMIDITY_ADC_DRY - HUMIDITY_ADC_WET);
}
float readVoltage() {
  float x = readAnalogFrom(ANA_SRC_VOLTAGE);
  return (x * ANA_VOLTAGE_REF * (VOLTAGE_DIVIDER_R1 + VOLTAGE_DIVIDER_R2)) / (VOLTAGE_DIVIDER_R1 * 1023);
}

void loop() {
  Serial.print(">> Humidity: "); Serial.print(readHumitiy(ANA_SRC_VOLTAGE)); Serial.println(" %");
  delay(2500);
  Serial.print(">> Voltage: "); Serial.print(readVoltage()); Serial.println(" V");
  delay(2500);
}
