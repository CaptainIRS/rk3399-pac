#[doc = "Register `EMMCCORE_CLKCTRL` reader"]
pub type R = crate::R<EmmccoreClkctrlSpec>;
#[doc = "Register `EMMCCORE_CLKCTRL` writer"]
pub type W = crate::W<EmmccoreClkctrlSpec>;
#[doc = "Field `INTERNALCLOCKENABLE` reader - 1: Oscillate 0: Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type InternalclockenableR = crate::BitReader;
#[doc = "Field `INTERNALCLOCKENABLE` writer - 1: Oscillate 0: Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
pub type InternalclockenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNALCLOCKSTABLE` reader - Internal Clock Stable. 1: Ready 0: Not Ready This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
pub type InternalclockstableR = crate::BitReader;
#[doc = "Field `SDCLKENA` reader - SD Clock Enable. 1: Enable 0: Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
pub type SdclkenaR = crate::BitReader;
#[doc = "Field `SDCLKENA` writer - SD Clock Enable. 1: Enable 0: Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
pub type SdclkenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGENSEL` reader - Clock Generator Select. 1: Programmable Clock Mode 0: Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
pub type ClkgenselR = crate::BitReader;
#[doc = "Field `CLKGENSEL` writer - Clock Generator Select. 1: Programmable Clock Mode 0: Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
pub type ClkgenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDCLKFREQSELUPPER` reader - Field0001 Abstract Upper Bits of SDCLK Frequency Select. Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
pub type SdclkfreqselupperR = crate::FieldReader;
#[doc = "Field `SDCLKFREQSELUPPER` writer - Field0001 Abstract Upper Bits of SDCLK Frequency Select. Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
pub type SdclkfreqselupperW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLKFREQSEL` reader - SDCLK Frequency Select. This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode 8'h80: base clock divided by 256 8'h40: base clock divided by 128 8'h20: base clock divided by 64 8'h10: base clock divided by 32 8'h08: base clock divided by 16 8'h04: base clock divided by 8 8'h02: base clock divided by 4 8'h01: base clock divided by 2 8'h00: base clock(10MHz-63MHz) Setting 8'h00 specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. a. 25 MHz divider value b. 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for eMMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for eMMC also. (2) 10-bit Divided Clock Mode Host Controller supports this mandatory mode instead of the 8- bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 10'h3FF:1/2046 Divided Clock N: 1/2N Divided Clock (Duty 50%) 10'h002: 1/4 Divided Clock 10'h001: 1/2 Divided Clock 10'h000: Base Clock (10MHz-254MHz)"]
pub type SdclkfreqselR = crate::FieldReader;
#[doc = "Field `SDCLKFREQSEL` writer - SDCLK Frequency Select. This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode 8'h80: base clock divided by 256 8'h40: base clock divided by 128 8'h20: base clock divided by 64 8'h10: base clock divided by 32 8'h08: base clock divided by 16 8'h04: base clock divided by 8 8'h02: base clock divided by 4 8'h01: base clock divided by 2 8'h00: base clock(10MHz-63MHz) Setting 8'h00 specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. a. 25 MHz divider value b. 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for eMMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for eMMC also. (2) 10-bit Divided Clock Mode Host Controller supports this mandatory mode instead of the 8- bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 10'h3FF:1/2046 Divided Clock N: 1/2N Divided Clock (Duty 50%) 10'h002: 1/4 Divided Clock 10'h001: 1/2 Divided Clock 10'h000: Base Clock (10MHz-254MHz)"]
pub type SdclkfreqselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 1: Oscillate 0: Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    pub fn internalclockenable(&self) -> InternalclockenableR {
        InternalclockenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable. 1: Ready 0: Not Ready This bit is set to 1 when SD clock is stable after writing to Internal Clock Enable in this register to 1. The SD Host Driver shall wait to set SD Clock Enable until this bit is set to 1. Note: This is useful when using PLL for a clock oscillator that requires setup time."]
    #[inline(always)]
    pub fn internalclockstable(&self) -> InternalclockstableR {
        InternalclockstableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable. 1: Enable 0: Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    pub fn sdclkena(&self) -> SdclkenaR {
        SdclkenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select. 1: Programmable Clock Mode 0: Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    pub fn clkgensel(&self) -> ClkgenselR {
        ClkgenselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Field0001 Abstract Upper Bits of SDCLK Frequency Select. Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqselupper(&self) -> SdclkfreqselupperR {
        SdclkfreqselupperR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select. This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode 8'h80: base clock divided by 256 8'h40: base clock divided by 128 8'h20: base clock divided by 64 8'h10: base clock divided by 32 8'h08: base clock divided by 16 8'h04: base clock divided by 8 8'h02: base clock divided by 4 8'h01: base clock divided by 2 8'h00: base clock(10MHz-63MHz) Setting 8'h00 specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. a. 25 MHz divider value b. 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for eMMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for eMMC also. (2) 10-bit Divided Clock Mode Host Controller supports this mandatory mode instead of the 8- bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 10'h3FF:1/2046 Divided Clock N: 1/2N Divided Clock (Duty 50%) 10'h002: 1/4 Divided Clock 10'h001: 1/2 Divided Clock 10'h000: Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SdclkfreqselR {
        SdclkfreqselR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Oscillate 0: Stop This bit is set to 0 when the HD is not using the HC or the HC awaits a wakeup event. The HC should stop its internal clock to go very low power state. Still, registers shall be able to be read and written. Clock starts to oscillate when this bit is set to 1. When clock oscillation is stable, the HC shall set Internal Clock Stable in this register to 1. This bit shall not affect card detection."]
    #[inline(always)]
    #[must_use]
    pub fn internalclockenable(&mut self) -> InternalclockenableW<EmmccoreClkctrlSpec> {
        InternalclockenableW::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock Enable. 1: Enable 0: Disable The HC shall stop SDCLK when writing this bit to 0. SDCLK frequency Select can be changed when this bit is 0. Then, the HC shall maintain the same clock frequency until SDCLK is stopped (Stop at SDCLK = 0). If the HC detects the No Card state, this bit shall be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn sdclkena(&mut self) -> SdclkenaW<EmmccoreClkctrlSpec> {
        SdclkenaW::new(self, 2)
    }
    #[doc = "Bit 5 - Clock Generator Select. 1: Programmable Clock Mode 0: Divided Clock Mode This bit is used to select the clock generator mode in SDCLK Frequency Select. If the Programmable Clock Mode is supported (non-zero value is set to Clock Multiplier in the Capabilities register), this bit attribute is RW, and if not supported, this bit attribute is RO and zero is read. This bit depends on the setting of Preset Value Enable in the Host Control 2 register. If the Preset Value Enable= 0, this bit is set by Host Driver. If the Preset Value Enable= 1, this bit is automatically set to a value specified in one of Preset Value registers."]
    #[inline(always)]
    #[must_use]
    pub fn clkgensel(&mut self) -> ClkgenselW<EmmccoreClkctrlSpec> {
        ClkgenselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Field0001 Abstract Upper Bits of SDCLK Frequency Select. Bit 07-06 is assigned to bit 09-08 of clock divider in SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqselupper(&mut self) -> SdclkfreqselupperW<EmmccoreClkctrlSpec> {
        SdclkfreqselupperW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select. This register is used to select the frequency of the SDCLK pin. The frequency is not programmed directly; rather this register holds the divisor of the Base Clock Frequency For SD clock in the capabilities register. Only the following settings are allowed. (1) 8-bit Divided Clock Mode 8'h80: base clock divided by 256 8'h40: base clock divided by 128 8'h20: base clock divided by 64 8'h10: base clock divided by 32 8'h08: base clock divided by 16 8'h04: base clock divided by 8 8'h02: base clock divided by 4 8'h01: base clock divided by 2 8'h00: base clock(10MHz-63MHz) Setting 8'h00 specifies the highest frequency of the SD Clock. When setting multiple bits, the most significant bit is used as the divisor. But multiple bits should not be set. The two default divider values can be calculated by the frequency that is defined by the Base Clock Frequency For SD Clock in the Capabilities register. a. 25 MHz divider value b. 400 KHz divider value The frequency of the SDCLK is set by the following formula: Clock Frequency = (Baseclock) / divisor. Thus choose the smallest possible divisor which results in a clock frequency that is less than or equal to the target frequency. Maximum Frequency for SD = 50Mhz (base clock) Maximum Frequency for eMMC = 52Mhz (base clock) Minimum Frequency = 195.3125Khz (50Mhz / 256), same calculation for eMMC also. (2) 10-bit Divided Clock Mode Host Controller supports this mandatory mode instead of the 8- bit Divided Clock Mode. The length of divider is extended to10 bits and all divider values shall be supported. 10'h3FF:1/2046 Divided Clock N: 1/2N Divided Clock (Duty 50%) 10'h002: 1/4 Divided Clock 10'h001: 1/2 Divided Clock 10'h000: Base Clock (10MHz-254MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfreqsel(&mut self) -> SdclkfreqselW<EmmccoreClkctrlSpec> {
        SdclkfreqselW::new(self, 8)
    }
}
#[doc = "Clock control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreClkctrlSpec;
impl crate::RegisterSpec for EmmccoreClkctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_clkctrl::R`](R) reader structure"]
impl crate::Readable for EmmccoreClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_clkctrl::W`](W) writer structure"]
impl crate::Writable for EmmccoreClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CLKCTRL to value 0"]
impl crate::Resettable for EmmccoreClkctrlSpec {
    const RESET_VALUE: u16 = 0;
}
