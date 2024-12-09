extern "C" {
    fn INT_AON_GPIO_EDGE();
    fn INT_I2C();
    fn INT_RFC_CPE_1();
    fn INT_PKA();
    fn INT_AON_RTC_COMB();
    fn INT_UART0_COMB();
    fn INT_AUX_SWEV0();
    fn INT_SSI0_COMB();
    fn INT_SSI1_COMB();
    fn INT_RFC_CPE_0();
    fn INT_RFC_HW_COMB();
    fn INT_RFC_CMD_ACK();
    fn INT_I2S();
    fn INT_AUX_SWEV1();
    fn INT_WDT();
    fn INT_GPT0A();
    fn INT_GPT0B();
    fn INT_GPT1A();
    fn INT_GPT1B();
    fn INT_GPT2A();
    fn INT_GPT2B();
    fn INT_GPT3A();
    fn INT_GPT3B();
    fn INT_CRYPTO_RESULT_AVAIL();
    fn INT_DMA_DONE_COMB();
    fn INT_DMA_ERR();
    fn INT_FLASH();
    fn INT_SWEV0();
    fn INT_AUX_COMB();
    fn INT_AON_PROG0();
    fn INT_PROG0();
    fn INT_AUX_COMPA();
    fn INT_AUX_ADC();
    fn INT_TRNG();
    fn INT_OSC_COMB();
    fn INT_AUX_TIMER2_EV0();
    fn INT_UART1_COMB();
    fn INT_BATMON_COMB();
    fn INT_SSI2_COMB();
    fn INT_SSI3_COMB();
    fn INT_UART2_COMB();
    fn INT_UART3_COMB();
    fn INT_I2C1();
    fn INT_SWEV1();
    fn INT_SWEV2();
    fn INT_SWEV3();
    fn INT_SWEV4();
}

pub static __INTERRUPTS: [Vector; 47] = [
    Vector { _handler: INT_AON_GPIO_EDGE },
    Vector { _handler: INT_I2C },
    Vector { _handler: INT_RFC_CPE_1 },
    Vector { _handler: INT_PKA },
    Vector { _handler: INT_AON_RTC_COMB },
    Vector { _handler: INT_UART0_COMB },
    Vector { _handler: INT_AUX_SWEV0 },
    Vector { _handler: INT_SSI0_COMB },
    Vector { _handler: INT_SSI1_COMB },
    Vector { _handler: INT_RFC_CPE_0 },
    Vector { _handler: INT_RFC_HW_COMB },
    Vector { _handler: INT_RFC_CMD_ACK },
    Vector { _handler: INT_I2S },
    Vector { _handler: INT_AUX_SWEV1 },
    Vector { _handler: INT_WDT },
    Vector { _handler: INT_GPT0A },
    Vector { _handler: INT_GPT0B },
    Vector { _handler: INT_GPT1A },
    Vector { _handler: INT_GPT1B },
    Vector { _handler: INT_GPT2A },
    Vector { _handler: INT_GPT2B },
    Vector { _handler: INT_GPT3A },
    Vector { _handler: INT_GPT3B },
    Vector { _handler: INT_CRYPTO_RESULT_AVAIL },
    Vector { _handler: INT_DMA_DONE_COMB },
    Vector { _handler: INT_DMA_ERR },
    Vector { _handler: INT_FLASH },
    Vector { _handler: INT_SWEV0 },
    Vector { _handler: INT_AUX_COMB },
    Vector { _handler: INT_AON_PROG0 },
    Vector { _handler: INT_PROG0 },
    Vector { _handler: INT_AUX_COMPA },
    Vector { _handler: INT_AUX_ADC },
    Vector { _handler: INT_TRNG },
    Vector { _handler: INT_OSC_COMB },
    Vector { _handler: INT_AUX_TIMER2_EV0 },
    Vector { _handler: INT_UART1_COMB },
    Vector { _handler: INT_BATMON_COMB },
    Vector { _handler: INT_SSI2_COMB },
    Vector { _handler: INT_SSI3_COMB },
    Vector { _handler: INT_UART2_COMB },
    Vector { _handler: INT_UART3_COMB },
    Vector { _handler: INT_I2C1 },
    Vector { _handler: INT_SWEV1 },
    Vector { _handler: INT_SWEV2 },
    Vector { _handler: INT_SWEV3 },
    Vector { _handler: INT_SWEV4 }
];

pub enum Interrupt {
    INT_AON_GPIO_EDGE = 0,
    INT_I2C = 1,
    INT_RFC_CPE_1 = 2,
    INT_PKA = 3,
    INT_AON_RTC_COMB = 4,
    INT_UART0_COMB = 5,
    INT_AUX_SWEV0 = 6,
    INT_SSI0_COMB = 7,
    INT_SSI1_COMB = 8,
    INT_RFC_CPE_0 = 9,
    INT_RFC_HW_COMB = 10,
    INT_RFC_CMD_ACK = 11,
    INT_I2S = 12,
    INT_AUX_SWEV1 = 13,
    INT_WDT = 14,
    INT_GPT0A = 15,
    INT_GPT0B = 16,
    INT_GPT1A = 17,
    INT_GPT1B = 18,
    INT_GPT2A = 19,
    INT_GPT2B = 20,
    INT_GPT3A = 21,
    INT_GPT3B = 22,
    INT_CRYPTO_RESULT_AVAIL = 23,
    INT_DMA_DONE_COMB = 24,
    INT_DMA_ERR = 25,
    INT_FLASH = 26,
    INT_SWEV0 = 27,
    INT_AUX_COMB = 28,
    INT_AON_PROG0 = 29,
    INT_PROG0 = 30,
    INT_AUX_COMPA = 31,
    INT_AUX_ADC = 32,
    INT_TRNG = 33,
    INT_OSC_COMB = 34,
    INT_AUX_TIMER2_EV0 = 35,
    INT_UART1_COMB = 36,
    INT_BATMON_COMB = 37,
    INT_SSI2_COMB = 38,
    INT_SSI3_COMB = 39,
    INT_UART2_COMB = 40,
    INT_UART3_COMB = 41,
    INT_I2C1 = 42,
    INT_SWEV1 = 43,
    INT_SWEV2 = 44,
    INT_SWEV3 = 45,
    INT_SWEV4 = 46,
}
