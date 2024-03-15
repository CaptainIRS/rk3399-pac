#[doc = "Register `USB3_GFLADJ` reader"]
pub type R = crate::R<Usb3GfladjSpec>;
#[doc = "Register `USB3_GFLADJ` writer"]
pub type W = crate::W<Usb3GfladjSpec>;
#[doc = "Field `GFLADJ_30MHZ` reader - GFLADJ_30MHZ This field indicates the value that is used for frame length adjustment instead of considering from the sideband input signal fladj_30mhz_reg. This enables post-silicon frame length adjustment in case the input signal fladj_30mhz_reg is connected to a wrong value or is not valid. For details on how to set this value, refer to section 5.2.4, \"Frame Length Adjustment Register (FLADJ),\" of the xHCI Specification."]
pub type Gfladj30mhzR = crate::FieldReader;
#[doc = "Field `GFLADJ_30MHZ` writer - GFLADJ_30MHZ This field indicates the value that is used for frame length adjustment instead of considering from the sideband input signal fladj_30mhz_reg. This enables post-silicon frame length adjustment in case the input signal fladj_30mhz_reg is connected to a wrong value or is not valid. For details on how to set this value, refer to section 5.2.4, \"Frame Length Adjustment Register (FLADJ),\" of the xHCI Specification."]
pub type Gfladj30mhzW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GFLADJ_30MHZ_SDBND_SEL` reader - GFLADJ_30MHZ_SDBND_SEL This field selects whether to use the input signal fladj_30mhz_reg or the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the SOF/ITP. When this bit is set to: 1, the controller uses the register field GFLADJ.GFLADJ_30MHZ value 0, the controller uses the input signal fladj_30mhz_reg value"]
pub type Gfladj30mhzSdbndSelR = crate::BitReader;
#[doc = "Field `GFLADJ_30MHZ_SDBND_SEL` writer - GFLADJ_30MHZ_SDBND_SEL This field selects whether to use the input signal fladj_30mhz_reg or the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the SOF/ITP. When this bit is set to: 1, the controller uses the register field GFLADJ.GFLADJ_30MHZ value 0, the controller uses the input signal fladj_30mhz_reg value"]
pub type Gfladj30mhzSdbndSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLADJ_REFCLK_FLADJ` reader - GFLADJ_REFCLK_FLADJ This field indicates the frame length adjustment to be applied when SOF/ITP counter is running on the ref_clk. This register value is used to adjust the ITP interval when GCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when GLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set to 1. The value is derived as follows: FLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)- (125000/ref_clk_period)) * ref_clk_period where: 1. The ref_clk_period_integer is the integer value of the ref_clk period got by truncating the decimal (fractional) value that is programmed in the GUCTL.REF_CLK_PERIOD field. 2. The ref_clk_period is the ref_clk period including the fractional value. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)- (125000/41.6666))*41.6666 = 2032 (ignoring the fractional value)"]
pub type GfladjRefclkFladjR = crate::FieldReader<u16>;
#[doc = "Field `GFLADJ_REFCLK_FLADJ` writer - GFLADJ_REFCLK_FLADJ This field indicates the frame length adjustment to be applied when SOF/ITP counter is running on the ref_clk. This register value is used to adjust the ITP interval when GCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when GLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set to 1. The value is derived as follows: FLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)- (125000/ref_clk_period)) * ref_clk_period where: 1. The ref_clk_period_integer is the integer value of the ref_clk period got by truncating the decimal (fractional) value that is programmed in the GUCTL.REF_CLK_PERIOD field. 2. The ref_clk_period is the ref_clk period including the fractional value. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)- (125000/41.6666))*41.6666 = 2032 (ignoring the fractional value)"]
pub type GfladjRefclkFladjW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `GFLADJ_REFCLK_LPM_SEL` reader - GFLADJ_REFCLK_LPM_SEL This bit enables the functionality of running SOF/ITP counters on the ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit is set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1, GCTL.SOFITPSYNC must not be set to 1. When GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of the suspend control of the USB 2.0 first port PHY (UTMI/ULPI) with USB 3.0 port states is removed. Note that the ref_clk frequencies supported in this mode are 16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the core must be connected to the FREECLK of the PHY. Note: If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
pub type GfladjRefclkLpmSelR = crate::BitReader;
#[doc = "Field `GFLADJ_REFCLK_LPM_SEL` writer - GFLADJ_REFCLK_LPM_SEL This bit enables the functionality of running SOF/ITP counters on the ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit is set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1, GCTL.SOFITPSYNC must not be set to 1. When GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of the suspend control of the USB 2.0 first port PHY (UTMI/ULPI) with USB 3.0 port states is removed. Note that the ref_clk frequencies supported in this mode are 16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the core must be connected to the FREECLK of the PHY. Note: If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
pub type GfladjRefclkLpmSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFLADJ_REFCLK_240MHZ_DECR` reader - GFLADJ_REFCLK_240MHZ_DECR This field indicates the decrement value that the controller applies for each ref_clk in order to derive a frame timer in terms of a 240-MHz clock. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1. The value is derived as follows: GFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
pub type GfladjRefclk240mhzDecrR = crate::FieldReader;
#[doc = "Field `GFLADJ_REFCLK_240MHZ_DECR` writer - GFLADJ_REFCLK_240MHZ_DECR This field indicates the decrement value that the controller applies for each ref_clk in order to derive a frame timer in terms of a 240-MHz clock. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1. The value is derived as follows: GFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
pub type GfladjRefclk240mhzDecrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GFLADJ_REFCLK_240MHZDECR_PLS1` reader - GFLADJ_REFCLK_240MHZDECR_PLS1 This field indicates that the decrement value that the controller applies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR and GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each ref_clk. Set this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1 and the fractional component of 240/ref_frequency is greater than or equal to 0.5. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10 3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
pub type GfladjRefclk240mhzdecrPls1R = crate::BitReader;
#[doc = "Field `GFLADJ_REFCLK_240MHZDECR_PLS1` writer - GFLADJ_REFCLK_240MHZDECR_PLS1 This field indicates that the decrement value that the controller applies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR and GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each ref_clk. Set this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1 and the fractional component of 240/ref_frequency is greater than or equal to 0.5. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10 3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
pub type GfladjRefclk240mhzdecrPls1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - GFLADJ_30MHZ This field indicates the value that is used for frame length adjustment instead of considering from the sideband input signal fladj_30mhz_reg. This enables post-silicon frame length adjustment in case the input signal fladj_30mhz_reg is connected to a wrong value or is not valid. For details on how to set this value, refer to section 5.2.4, \"Frame Length Adjustment Register (FLADJ),\" of the xHCI Specification."]
    #[inline(always)]
    pub fn gfladj_30mhz(&self) -> Gfladj30mhzR {
        Gfladj30mhzR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - GFLADJ_30MHZ_SDBND_SEL This field selects whether to use the input signal fladj_30mhz_reg or the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the SOF/ITP. When this bit is set to: 1, the controller uses the register field GFLADJ.GFLADJ_30MHZ value 0, the controller uses the input signal fladj_30mhz_reg value"]
    #[inline(always)]
    pub fn gfladj_30mhz_sdbnd_sel(&self) -> Gfladj30mhzSdbndSelR {
        Gfladj30mhzSdbndSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:21 - GFLADJ_REFCLK_FLADJ This field indicates the frame length adjustment to be applied when SOF/ITP counter is running on the ref_clk. This register value is used to adjust the ITP interval when GCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when GLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set to 1. The value is derived as follows: FLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)- (125000/ref_clk_period)) * ref_clk_period where: 1. The ref_clk_period_integer is the integer value of the ref_clk period got by truncating the decimal (fractional) value that is programmed in the GUCTL.REF_CLK_PERIOD field. 2. The ref_clk_period is the ref_clk period including the fractional value. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)- (125000/41.6666))*41.6666 = 2032 (ignoring the fractional value)"]
    #[inline(always)]
    pub fn gfladj_refclk_fladj(&self) -> GfladjRefclkFladjR {
        GfladjRefclkFladjR::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bit 23 - GFLADJ_REFCLK_LPM_SEL This bit enables the functionality of running SOF/ITP counters on the ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit is set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1, GCTL.SOFITPSYNC must not be set to 1. When GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of the suspend control of the USB 2.0 first port PHY (UTMI/ULPI) with USB 3.0 port states is removed. Note that the ref_clk frequencies supported in this mode are 16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the core must be connected to the FREECLK of the PHY. Note: If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
    #[inline(always)]
    pub fn gfladj_refclk_lpm_sel(&self) -> GfladjRefclkLpmSelR {
        GfladjRefclkLpmSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - GFLADJ_REFCLK_240MHZ_DECR This field indicates the decrement value that the controller applies for each ref_clk in order to derive a frame timer in terms of a 240-MHz clock. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1. The value is derived as follows: GFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
    #[inline(always)]
    pub fn gfladj_refclk_240mhz_decr(&self) -> GfladjRefclk240mhzDecrR {
        GfladjRefclk240mhzDecrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - GFLADJ_REFCLK_240MHZDECR_PLS1 This field indicates that the decrement value that the controller applies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR and GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each ref_clk. Set this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1 and the fractional component of 240/ref_frequency is greater than or equal to 0.5. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10 3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
    #[inline(always)]
    pub fn gfladj_refclk_240mhzdecr_pls1(&self) -> GfladjRefclk240mhzdecrPls1R {
        GfladjRefclk240mhzdecrPls1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - GFLADJ_30MHZ This field indicates the value that is used for frame length adjustment instead of considering from the sideband input signal fladj_30mhz_reg. This enables post-silicon frame length adjustment in case the input signal fladj_30mhz_reg is connected to a wrong value or is not valid. For details on how to set this value, refer to section 5.2.4, \"Frame Length Adjustment Register (FLADJ),\" of the xHCI Specification."]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_30mhz(&mut self) -> Gfladj30mhzW<Usb3GfladjSpec> {
        Gfladj30mhzW::new(self, 0)
    }
    #[doc = "Bit 7 - GFLADJ_30MHZ_SDBND_SEL This field selects whether to use the input signal fladj_30mhz_reg or the GFLADJ.GFLADJ_30MHZ to adjust the frame length for the SOF/ITP. When this bit is set to: 1, the controller uses the register field GFLADJ.GFLADJ_30MHZ value 0, the controller uses the input signal fladj_30mhz_reg value"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_30mhz_sdbnd_sel(&mut self) -> Gfladj30mhzSdbndSelW<Usb3GfladjSpec> {
        Gfladj30mhzSdbndSelW::new(self, 7)
    }
    #[doc = "Bits 8:21 - GFLADJ_REFCLK_FLADJ This field indicates the frame length adjustment to be applied when SOF/ITP counter is running on the ref_clk. This register value is used to adjust the ITP interval when GCTL\\[SOFITPSYNC\\]
is set to 1; SOF and ITP interval when GLADJ.GFLADJ_REFCLK_LPM_SEL is set to 1. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1 or GCTL.SOFITPSYNC is set to 1. The value is derived as follows: FLADJ_REF_CLK_FLADJ=((125000/ref_clk_period_integer)- (125000/ref_clk_period)) * ref_clk_period where: 1. The ref_clk_period_integer is the integer value of the ref_clk period got by truncating the decimal (fractional) value that is programmed in the GUCTL.REF_CLK_PERIOD field. 2. The ref_clk_period is the ref_clk period including the fractional value. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GLADJ_REFCLK_FLADJ = ((125000/41)- (125000/41.6666))*41.6666 = 2032 (ignoring the fractional value)"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_fladj(&mut self) -> GfladjRefclkFladjW<Usb3GfladjSpec> {
        GfladjRefclkFladjW::new(self, 8)
    }
    #[doc = "Bit 23 - GFLADJ_REFCLK_LPM_SEL This bit enables the functionality of running SOF/ITP counters on the ref_clk. This bit must not be set to 1 if GCTL.SOFITPSYNC bit is set to 1. Similarly, if GFLADJ_REFCLK_LPM_SEL set to 1, GCTL.SOFITPSYNC must not be set to 1. When GFLADJ_REFCLK_LPM_SEL is set to 1 the overloading of the suspend control of the USB 2.0 first port PHY (UTMI/ULPI) with USB 3.0 port states is removed. Note that the ref_clk frequencies supported in this mode are 16/17/19.2/20/24/39.7/40 MHz. The utmi_clk\\[0\\]
signal of the core must be connected to the FREECLK of the PHY. Note: If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit must be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_lpm_sel(&mut self) -> GfladjRefclkLpmSelW<Usb3GfladjSpec> {
        GfladjRefclkLpmSelW::new(self, 23)
    }
    #[doc = "Bits 24:30 - GFLADJ_REFCLK_240MHZ_DECR This field indicates the decrement value that the controller applies for each ref_clk in order to derive a frame timer in terms of a 240-MHz clock. This field must be programmed to a non-zero value only if GFLADJ_REFCLK_LPM_SEL is set to 1. The value is derived as follows: GFLADJ_REFCLK_240MHZ_DECR = 240/ref_clk_frequency Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = 240/24 = 10"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_240mhz_decr(&mut self) -> GfladjRefclk240mhzDecrW<Usb3GfladjSpec> {
        GfladjRefclk240mhzDecrW::new(self, 24)
    }
    #[doc = "Bit 31 - GFLADJ_REFCLK_240MHZDECR_PLS1 This field indicates that the decrement value that the controller applies for each ref_clk must be GFLADJ_REFCLK_240MHZ_DECR and GFLADJ_REFCLK_240MHZ_DECR +1 alternatively on each ref_clk. Set this bit to a 1 only if GFLADJ_REFCLK_LPM_SEL is set to 1 and the fractional component of 240/ref_frequency is greater than or equal to 0.5. Examples: If the ref_clk is 24 MHz then 1. GUCTL.REF_CLK_PERIOD = 41 2. GFLADJ.GFLADJ_REFCLK_240MHZ_DECR = (240/24) = 10 3. GFLADJ.GFLADJ_REFCLK_240MHZDECR_PLS1 = 0"]
    #[inline(always)]
    #[must_use]
    pub fn gfladj_refclk_240mhzdecr_pls1(&mut self) -> GfladjRefclk240mhzdecrPls1W<Usb3GfladjSpec> {
        GfladjRefclk240mhzdecrPls1W::new(self, 31)
    }
}
#[doc = "Global Frame Length Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gfladj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gfladj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GfladjSpec;
impl crate::RegisterSpec for Usb3GfladjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gfladj::R`](R) reader structure"]
impl crate::Readable for Usb3GfladjSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gfladj::W`](W) writer structure"]
impl crate::Writable for Usb3GfladjSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GFLADJ to value 0"]
impl crate::Resettable for Usb3GfladjSpec {
    const RESET_VALUE: u32 = 0;
}
