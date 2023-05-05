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
#define MUX_C   D0

#define ANA_SRC_VOLTAGE    6
#define ANA_SRC_HUMID_0    5
#define ANA_SRC_HUMID_1    3
#define ANA_SRC_HUMID_2    1
#define ANA_SRC_HUMID_3    7
#define ANA_SRC_HUMID_4    2
#define ANA_SRC_HUMID_5    0

#define ANA_VOLTAGE_REF    3.3     // Reference voltage corresponding to 1023 of ADC [V]

#define HUMIDITY_ADC_DRY   800     // ADC value when reading complete dry soil [0..1023]
#define HUMIDITY_ADC_WET   200     // ADC value when reading complete wet soid [0..1023] 
#define VOLTAGE_DIVIDER_R1 92e3    // Low-side resistor of voltage divider [Ohm]
#define VOLTAGE_DIVIDER_R2 48e3    // High-side resistor of voltage divider [Ohm]

char ANA_SRC_HUMIDS[6] = {
  ANA_SRC_HUMID_0,
  ANA_SRC_HUMID_1,
  ANA_SRC_HUMID_2,
  ANA_SRC_HUMID_3,
  ANA_SRC_HUMID_4,
  ANA_SRC_HUMID_5
};

void setup() {
  Serial.begin(9600);
  pinMode(MUX_A, OUTPUT);
  pinMode(MUX_B, OUTPUT);
  pinMode(MUX_C, OUTPUT);
  pinMode(A0, INPUT);
}


int readAnalogFrom(char channel) {
  digitalWrite(MUX_A, channel & (1 << 0));
  digitalWrite(MUX_B, channel & (1 << 1));
  digitalWrite(MUX_C, channel & (1 << 2));
  return analogRead(A0);
}

float readHumitiy(char channel) {
  int x = readAnalogFrom(channel);
  if(x > HUMIDITY_ADC_DRY + 25 || x < HUMIDITY_ADC_WET - 25) {
    return NAN;
  }
  
  return 100.0 - 100.0 * ((float)x - HUMIDITY_ADC_WET) / (HUMIDITY_ADC_DRY - HUMIDITY_ADC_WET);
}
float readVoltage() {
  float x = readAnalogFrom(ANA_SRC_VOLTAGE);
  return (x * ANA_VOLTAGE_REF * (VOLTAGE_DIVIDER_R1 + VOLTAGE_DIVIDER_R2)) / (VOLTAGE_DIVIDER_R1 * 1023);
}

void loop() {
  Serial.print(">> Voltage: "); Serial.print(readVoltage()); Serial.println(" V");
  for (int i = 0; i < 6; i++) {
    Serial.print(">> Humidity #: "); Serial.print(i); Serial.print(": "); 
    Serial.print(readHumitiy(ANA_SRC_HUMIDS[i]), 0); 
    Serial.println(" %");
  }
  
  delay(2500);
}
