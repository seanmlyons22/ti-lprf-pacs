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
