extern "C" {
    fn INT_AON_GPIO_EDGE();
    fn INT_I2C();
    fn INT_RFC_CPE_1();
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
}

pub static __INTERRUPTS: [Vector; 33] = [
    Vector { _handler: INT_AON_GPIO_EDGE },
    Vector { _handler: INT_I2C },
    Vector { _handler: INT_RFC_CPE_1 },
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
    Vector { _handler: INT_TRNG }
];

pub enum Interrupt {
    INT_AON_GPIO_EDGE = 0,
    INT_I2C = 1,
    INT_RFC_CPE_1 = 2,
    INT_AON_RTC_COMB = 3,
    INT_UART0_COMB = 4,
    INT_AUX_SWEV0 = 5,
    INT_SSI0_COMB = 6,
    INT_SSI1_COMB = 7,
    INT_RFC_CPE_0 = 8,
    INT_RFC_HW_COMB = 9,
    INT_RFC_CMD_ACK = 10,
    INT_I2S = 11,
    INT_AUX_SWEV1 = 12,
    INT_WDT = 13,
    INT_GPT0A = 14,
    INT_GPT0B = 15,
    INT_GPT1A = 16,
    INT_GPT1B = 17,
    INT_GPT2A = 18,
    INT_GPT2B = 19,
    INT_GPT3A = 20,
    INT_GPT3B = 21,
    INT_CRYPTO_RESULT_AVAIL = 22,
    INT_DMA_DONE_COMB = 23,
    INT_DMA_ERR = 24,
    INT_FLASH = 25,
    INT_SWEV0 = 26,
    INT_AUX_COMB = 27,
    INT_AON_PROG0 = 28,
    INT_PROG0 = 29,
    INT_AUX_COMPA = 30,
    INT_AUX_ADC = 31,
    INT_TRNG = 32,
}
