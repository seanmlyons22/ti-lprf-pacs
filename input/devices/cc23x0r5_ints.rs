extern "C" {
    fn INT_CPUIRQ0();
    fn INT_CPUIRQ1();
    fn INT_CPUIRQ2();
    fn INT_CPUIRQ3();
    fn INT_CPUIRQ4();
    fn INT_GPIO_COMB();
    fn INT_LRFD_IRQ0();
    fn INT_LRFD_IRQ1();
    fn INT_DMA_DONE_COMB();
    fn INT_AES_COMB();
    fn INT_SPI0_COMB();
    fn INT_UART0_COMB();
    fn INT_I2C0_IRQ();
    fn INT_LGPT0_COMB();
    fn INT_LGPT1_COMB();
    fn INT_ADC_COMB();
    fn INT_CPUIRQ16();
    fn INT_LGPT2_COMB();
    fn INT_LGPT3_COMB();
}

pub static __INTERRUPTS: [Vector; 19] = [
    Vector { _handler: INT_CPUIRQ0 },
    Vector { _handler: INT_CPUIRQ1 },
    Vector { _handler: INT_CPUIRQ2 },
    Vector { _handler: INT_CPUIRQ3 },
    Vector { _handler: INT_CPUIRQ4 },
    Vector { _handler: INT_GPIO_COMB },
    Vector { _handler: INT_LRFD_IRQ0 },
    Vector { _handler: INT_LRFD_IRQ1 },
    Vector { _handler: INT_DMA_DONE_COMB },
    Vector { _handler: INT_AES_COMB },
    Vector { _handler: INT_SPI0_COMB },
    Vector { _handler: INT_UART0_COMB },
    Vector { _handler: INT_I2C0_IRQ },
    Vector { _handler: INT_LGPT0_COMB },
    Vector { _handler: INT_LGPT1_COMB },
    Vector { _handler: INT_ADC_COMB },
    Vector { _handler: INT_CPUIRQ16 },
    Vector { _handler: INT_LGPT2_COMB },
    Vector { _handler: INT_LGPT3_COMB }
];

pub enum Interrupt {
    INT_CPUIRQ0 = 0,
    INT_CPUIRQ1 = 1,
    INT_CPUIRQ2 = 2,
    INT_CPUIRQ3 = 3,
    INT_CPUIRQ4 = 4,
    INT_GPIO_COMB = 5,
    INT_LRFD_IRQ0 = 6,
    INT_LRFD_IRQ1 = 7,
    INT_DMA_DONE_COMB = 8,
    INT_AES_COMB = 9,
    INT_SPI0_COMB = 10,
    INT_UART0_COMB = 11,
    INT_I2C0_IRQ = 12,
    INT_LGPT0_COMB = 13,
    INT_LGPT1_COMB = 14,
    INT_ADC_COMB = 15,
    INT_CPUIRQ16 = 16,
    INT_LGPT2_COMB = 17,
    INT_LGPT3_COMB = 18,
}